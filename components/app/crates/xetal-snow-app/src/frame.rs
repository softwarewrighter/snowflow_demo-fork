use crate::{Scene, status};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};
use xetal_world_audio::{Kind, Sound};
use xetal_world_render::Renderer;

type Callback = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;
pub(crate) struct App {
    pub scene: Scene,
    renderer: Renderer,
    canvas: web_sys::HtmlCanvasElement,
    pub started: bool,
    pub effects: [bool; 2],
    pub audio: Sound,
    meter: xetal_world_metrics::Meter,
    last_frame: f64,
    frames: u32,
    updates: u32,
}

impl App {
    pub fn new(scene: Scene, renderer: Renderer, canvas: web_sys::HtmlCanvasElement) -> Self {
        Self {
            scene,
            renderer,
            canvas,
            started: false,
            effects: [true; 2],
            audio: Sound::default(),
            meter: Default::default(),
            last_frame: 0.0,
            frames: 0,
            updates: 0,
        }
    }

    fn tick(&mut self, now: f64) -> Result<(), String> {
        let dt = ((now - self.last_frame) / 1000.0) as f32;
        self.last_frame = now;
        let (meter, reading) = self.meter.record(now);
        self.meter = meter;
        if let Some(reading) = reading {
            status::text(
                "fps",
                &format!("{:.0} FPS / {:.1} ms", reading.fps, reading.mean_ms),
            );
            if let Ok(node) = status::element("fps") {
                let _ = node.set_attribute(
                    "title",
                    &format!("Worst recent frame: {:.1} ms", reading.worst_ms),
                );
            }
        }
        self.advance(dt)?;
        crate::boot::resize(&self.canvas, &mut self.renderer);
        let walker = &self.scene.walker;
        self.renderer
            .draw(walker.yaw, walker.pitch, walker.position, self.effects)?;
        self.frames += 1;
        if self.frames.is_multiple_of(6) {
            self.observe();
        }
        Ok(())
    }

    fn advance(&mut self, dt: f32) -> Result<(), String> {
        let update = self.scene.update(dt, !self.started)?;
        if let Some(mesh) = update.terrain {
            self.renderer.upload(&mesh)?;
            self.updates += 1;
        }
        if let Some(vertices) = update.tracks {
            self.renderer.upload_tracks(&vertices)?;
        }
        for resource in update.resources {
            match resource.material.as_str() {
                "ice" => self.renderer.upload_objects(resource.mesh.positions())?,
                "slush" => self.renderer.upload_sweep(resource.mesh.positions())?,
                "water" => self.renderer.upload_ribbon(resource.mesh.positions())?,
                _ => return Err("unsupported material".into()),
            }
        }
        if let Some(rows) = update.particles {
            self.renderer.upload_particles(&rows)?;
        }
        let sound = match update.sound[0] {
            1.0 => self
                .audio
                .play(Kind::Step, update.sound[1], update.sound[2]),
            2.0 => self
                .audio
                .play(Kind::Land, update.sound[1], update.sound[2]),
            _ => Ok(()),
        };
        if let Err(error) = sound {
            self.audio.mute();
            status::text("sound-status", &format!("Sound unavailable: {error}"));
        }
        if !update.moving {
            self.audio.quiet_steps();
        }
        Ok(())
    }

    fn observe(&self) {
        let scene = &self.scene;
        let chunks = scene.world.resident();
        let visible = scene
            .trail
            .marks
            .iter()
            .filter(|mark| mark.opacity > 0.01)
            .count();
        for button in &scene.frame.buttons {
            status::text(&button.id, &button.label);
        }
        if !self.started {
            let loaded = chunks;
            status::text(
                "splash-status",
                &format!("Preparing nearby snow ({loaded}/25)..."),
            );
            if loaded == 25 {
                status::text("splash-status", "Ready. Choose how to enter.");
                for id in ["start-sound", "start-muted"] {
                    if let Ok(button) = status::element(id) {
                        let _ = button.remove_attribute("disabled");
                    }
                }
            }
        }
        if chunks > 0 {
            status::progress(self.frames, self.updates, scene.mesh_hash);
        }
        status::text(
            "triangles",
            &format!(
                "{}",
                chunks * 512
                    + scene
                        .frame
                        .resources
                        .iter()
                        .map(|r| r.mesh.indices().len() / 3)
                        .sum::<usize>()
            ),
        );
        status::text("updates", &format!("{chunks} chunks / {visible} prints"));
        self.attributes();
    }
    fn attributes(&self) {
        let scene = &self.scene;
        let chunks = scene.world.resident();
        let visible = scene
            .trail
            .marks
            .iter()
            .filter(|mark| mark.opacity > 0.01)
            .count();
        if let Ok(element) = status::element("app") {
            for (key, value) in [
                ("data-position", format!("{:?}", scene.walker.position)),
                ("data-chunks", chunks.to_string()),
                (
                    "data-center",
                    format!("[{},{}]", scene.world.center.0, scene.world.center.1),
                ),
                ("data-footprints", visible.to_string()),
                ("data-steps", scene.steps.to_string()),
                ("data-landings", scene.landings.to_string()),
                ("data-wind", scene.wind.to_string()),
                ("data-edits", scene.world.edit_revision.to_string()),
                ("data-audio", self.audio.state().to_string()),
                ("data-grounded", scene.walker.motion.grounded.to_string()),
            ] {
                let _ = element.set_attribute(key, &value);
            }
            for (key, value) in &scene.frame.stats {
                let _ = element.set_attribute(&format!("data-{key}"), &value.to_string());
            }
        }
    }
}

pub(crate) fn run(state: Rc<RefCell<App>>) -> Result<(), String> {
    let callback: Callback = Rc::new(RefCell::new(None));
    let next = callback.clone();
    *callback.borrow_mut() = Some(Closure::new(move |now| {
        if status::failed() {
            return;
        }
        if let Err(error) = state.borrow_mut().tick(now) {
            status::error(error);
            return;
        }
        if let Some(callback) = next.borrow().as_ref()
            && let Some(window) = web_sys::window()
        {
            let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
        }
    }));
    let window = web_sys::window().ok_or("no browser window")?;
    let borrowed = callback.borrow();
    let handler = borrowed.as_ref().ok_or("missing frame callback")?;
    window
        .request_animation_frame(handler.as_ref().unchecked_ref())
        .map_err(|e| format!("{e:?}"))?;
    Ok(())
}
