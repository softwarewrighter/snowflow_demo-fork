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
    pub paused: bool,
    pub started: bool,
    pub held: [bool; 4],
    pub effects: [bool; 2],
    pub jump: bool,
    pub audio: Sound,
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
            paused: false,
            started: false,
            held: [false; 4],
            effects: [true; 2],
            jump: false,
            audio: Sound::default(),
            last_frame: 0.0,
            frames: 0,
            updates: 0,
        }
    }

    fn tick(&mut self, now: f64) -> Result<(), String> {
        let dt = ((now - self.last_frame) / 1000.0) as f32;
        self.last_frame = now;
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
        let update = self
            .scene
            .update(dt, self.held, self.jump, self.paused || !self.started)?;
        self.jump = false;
        if let Some(mesh) = update.terrain {
            self.renderer.upload(&mesh)?;
            self.updates += 1;
        }
        if let Some(vertices) = update.tracks {
            self.renderer.upload_tracks(&vertices)?;
        }
        let sound = if update.landing > 0.0 {
            self.audio.play(Kind::Land, update.landing / 8.0)
        } else if update.step {
            self.audio.play(Kind::Step, 1.0)
        } else {
            Ok(())
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
        if !self.started {
            let loaded = scene.world.resident();
            status::text("splash-status", &format!("Preparing nearby snow ({loaded}/25)..."));
            if loaded == 25 {
                status::text("splash-status", "Ready. Choose how to enter.");
                for id in ["start-sound", "start-muted"] {
                    if let Ok(button) = status::element(id) { let _ = button.remove_attribute("disabled"); }
                }
            }
        }
        if scene.world.resident() > 0 {
            status::progress(self.frames, self.updates, scene.mesh_hash);
        }
        status::text("triangles", &format!("{}", scene.world.resident() * 512));
        status::text(
            "updates",
            &format!(
                "{} chunks / {} prints",
                scene.world.resident(),
                scene.trail.marks.len()
            ),
        );
        if let Ok(element) = status::element("app") {
            for (key, value) in [
                ("data-position", format!("{:?}", scene.walker.position)),
                ("data-chunks", scene.world.resident().to_string()),
                (
                    "data-center",
                    format!("[{},{}]", scene.world.center.0, scene.world.center.1),
                ),
                ("data-footprints", scene.trail.marks.len().to_string()),
                ("data-steps", scene.trail.steps.to_string()),
                ("data-landings", scene.landings.to_string()),
                ("data-audio", self.audio.state().to_string()),
                ("data-grounded", scene.walker.motion.grounded.to_string()),
            ] {
                let _ = element.set_attribute(key, &value);
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
