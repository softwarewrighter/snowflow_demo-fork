use crate::status;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};
use xetal_world_model::{Demo, Progress};
use xetal_world_render::Renderer;

type Callback = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;
pub(crate) struct App {
    demo: Demo,
    renderer: Renderer,
    canvas: web_sys::HtmlCanvasElement,
    pub paused: bool,
    pub pending: Option<String>,
    pub walker: xetal_world_navigation::Walker,
    pub held: [bool; 4],
    terrain: Vec<f32>,
    last_frame: f64,
    waiting: bool,
    last_tick: f64,
    frames: u32,
    updates: u32,
    mesh_hash: u32,
}

impl App {
    pub fn new(demo: Demo, renderer: Renderer, canvas: web_sys::HtmlCanvasElement) -> Self {
        Self {
            demo,
            renderer,
            canvas,
            paused: false,
            pending: None,
            walker: Default::default(),
            held: [false; 4],
            terrain: Vec::new(),
            last_frame: 0.0,
            waiting: false,
            last_tick: 0.0,
            frames: 0,
            updates: 0,
            mesh_hash: 0,
        }
    }

    fn tick(&mut self, now: f64) -> Result<(), String> {
        if self.waiting {
            let tick =
                (!self.paused && now - self.last_tick >= 80.0).then(|| "tick 0.08".to_string());
            if let Some(event) = self.pending.take().or(tick) {
                self.demo.event(&event)?;
                self.waiting = false;
                self.last_tick = now;
            }
        }
        self.advance()?;
        crate::boot::resize(&self.canvas, &mut self.renderer);
        self.navigate(now);
        self.renderer
            .draw(self.walker.yaw, self.walker.pitch, self.walker.position)?;
        self.frames += 1;
        if self.waiting {
            status::progress(self.frames, self.updates, self.mesh_hash);
        }
        Ok(())
    }
    fn navigate(&mut self, now: f64) {
        let dt = ((now - self.last_frame) / 1000.0) as f32;
        self.last_frame = now;
        if !self.paused {
            let forward = i32::from(self.held[0]) - i32::from(self.held[1]);
            let sideways = i32::from(self.held[3]) - i32::from(self.held[2]);
            self.walker.advance(dt, forward as f32, sideways as f32);
        }
        self.walker.ground(&self.terrain);
        if let Ok(element) = status::element("app") {
            let _ = element.set_attribute("data-position", &format!("{:?}", self.walker.position));
        }
    }

    fn advance(&mut self) -> Result<(), String> {
        if !self.waiting {
            match self.demo.step(8_000)? {
                Progress::Waiting => {
                    let mesh = self.demo.mesh()?;
                    self.mesh_hash = mesh.positions().iter().fold(2166136261_u32, |hash, value| {
                        (hash ^ value.to_bits()).wrapping_mul(16777619)
                    });
                    self.renderer.upload(&mesh)?;
                    self.terrain = mesh.positions().to_vec();
                    self.waiting = true;
                    self.updates += 1;
                }
                Progress::Done => return Err("The X_eTaL event loop ended.".into()),
                Progress::Running => {}
            }
        }
        Ok(())
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
