use crate::{frame, input, status};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use xetal_world_model::Demo;
use xetal_world_render::Renderer;

pub(crate) async fn start() -> Result<(), String> {
    let window = web_sys::window().ok_or("no browser window")?;
    let gpu = js_sys::Reflect::get(&window.navigator(), &"gpu".into()).unwrap_or_default();
    if gpu.is_undefined() || gpu.is_null() {
        return Err("WebGPU is unavailable. Use a supported browser and GPU.".into());
    }
    status::text("source", crate::SOURCE);
    let canvas = canvas()?;
    status::text("status", "Creating WebGPU device...");
    let renderer = Renderer::new(canvas.clone(), status::error).await?;
    let demo = Demo::new()?;
    let state = Rc::new(RefCell::new(frame::App::new(demo, renderer, canvas)));
    input::install(state.clone())?;
    frame::run(state)
}

pub(crate) fn resize(canvas: &web_sys::HtmlCanvasElement, renderer: &mut Renderer) {
    let scale = web_sys::window()
        .map(|w| w.device_pixel_ratio().min(2.0))
        .unwrap_or(1.0);
    let width = (f64::from(canvas.client_width().max(1)) * scale) as u32;
    let height = (f64::from(canvas.client_height().max(1)) * scale) as u32;
    if canvas.width() != width {
        canvas.set_width(width);
    }
    if canvas.height() != height {
        canvas.set_height(height);
    }
    renderer.resize(width, height);
}

pub(crate) fn canvas() -> Result<web_sys::HtmlCanvasElement, String> {
    status::element("scene")?
        .dyn_into()
        .map_err(|_| "scene is not a canvas".into())
}
