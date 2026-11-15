use crate::Scene;
use crate::{frame, input, status};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use xetal_world_render::Renderer;

pub(crate) async fn start() -> Result<(), String> {
    status::text(
        "build-info",
        &format!(
            "Build ({} {} {})",
            env!("BUILD_HOST"),
            env!("BUILD_SHA"),
            env!("BUILD_TIMESTAMP")
        ),
    );
    let window = web_sys::window().ok_or("no browser window")?;
    let gpu = js_sys::Reflect::get(&window.navigator(), &"gpu".into()).unwrap_or_default();
    if gpu.is_undefined() || gpu.is_null() {
        return Err("WebGPU is unavailable. Use a supported browser and GPU.".into());
    }
    let canvas = canvas()?;
    status::text("status", "Creating WebGPU device...");
    let renderer = Renderer::new(canvas.clone(), status::error).await?;
    let scene = Scene::new()?;
    views(&scene)?;
    let state = Rc::new(RefCell::new(frame::App::new(scene, renderer, canvas)));
    input::install(state.clone())?;
    status::text("splash-status", "Generating nearby snow...");
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

pub(crate) fn enter(app: &mut frame::App, with_sound: bool) {
    app.started = true;
    if with_sound {
        if !app.audio.enabled {
            sound(app);
        }
    } else {
        app.audio.mute();
        status::text("sound", "Sound: off");
    }
    if let Ok(splash) = status::element("splash") {
        let _ = splash.set_attribute("hidden", "");
    }
    if let Ok(canvas) = canvas() {
        let _ = canvas.focus();
    }
}

pub(crate) fn sound(app: &mut frame::App) {
    if app.audio.enabled {
        app.audio.mute();
        status::text("sound", "Sound: off");
        return;
    }
    match app.audio.enable() {
        Ok(promise) => {
            status::text("sound", "Sound: on");
            wasm_bindgen_futures::spawn_local(async move {
                if wasm_bindgen_futures::JsFuture::from(promise).await.is_err() {
                    status::text(
                        "sound-status",
                        "Audio was blocked. Use Sound to retry, or continue muted.",
                    );
                }
            });
        }
        Err(error) => status::text("sound-status", &format!("Audio unavailable: {error}")),
    }
}

fn views(scene: &Scene) -> Result<(), String> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or("no document")?;
    let controls = status::element("game-controls")?;
    for button in &scene.frame.buttons {
        let node = document
            .create_element("button")
            .map_err(|e| format!("{e:?}"))?;
        node.set_id(&button.id);
        node.set_text_content(Some(&button.label));
        controls.append_child(&node).map_err(|e| format!("{e:?}"))?;
    }
    let panels = status::element("source-panels")?;
    for source in &scene.frame.sources {
        let panel = source_panel(&document, source)?;
        panels.append_child(&panel).map_err(|e| format!("{e:?}"))?;
    }
    Ok(())
}

fn source_panel(
    document: &web_sys::Document,
    source: &xetal_app_session::Source,
) -> Result<web_sys::Element, String> {
    let text = if source.name == "entry" {
        crate::SOURCE
    } else {
        xetal_snow_libraries::LIBRARIES
            .iter()
            .find(|l| l.name == source.name)
            .ok_or("unknown source module")?
            .source
    };
    let panel = document
        .create_element("details")
        .map_err(|e| format!("{e:?}"))?;
    panel.set_inner_html(r#"<summary></summary><div class="source-views"><section><h2>As typed</h2><pre tabindex="0"></pre></section><section><h2>As rendered</h2><pre tabindex="0" class="rendered-source"></pre></section></div>"#);
    panel
        .query_selector("summary")
        .map_err(|e| format!("{e:?}"))?
        .ok_or("missing summary")?
        .set_text_content(Some(&source.title));
    let typed = panel
        .query_selector("section:first-child pre")
        .map_err(|e| format!("{e:?}"))?
        .ok_or("missing typed source")?;
    typed.set_text_content(Some(text));
    let rendered = panel
        .query_selector(".rendered-source")
        .map_err(|e| format!("{e:?}"))?
        .ok_or("missing rendered source")?;
    rendered.set_inner_html(&xetal_world_model::highlighted_text(text));
    if source.name == "entry" {
        typed.set_id("source");
        rendered.set_id("source-rendered");
    }
    Ok(panel)
}
