use crate::{frame::App, status};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};

type State = Rc<RefCell<App>>;

pub(crate) fn install(state: State) -> Result<(), String> {
    button("pause", state.clone(), |app| {
        app.paused = !app.paused;
        status::text("pause", if app.paused { "Resume" } else { "Pause" });
    })?;
    button("reset", state.clone(), |app| {
        app.pending = Some("key Home".into())
    })?;
    button("raise", state.clone(), |app| {
        app.pending = Some("down 0 0".into())
    })?;
    button("scene", state.clone(), |app| {
        app.pending = Some("down 0 0".into())
    })?;
    keys(state)
}

fn button(id: &str, state: State, action: fn(&mut App)) -> Result<(), String> {
    let handler =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |_| action(&mut state.borrow_mut()));
    status::element(id)?
        .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .map_err(|e| format!("{e:?}"))?;
    handler.forget();
    Ok(())
}

fn keys(state: State) -> Result<(), String> {
    let handler =
        Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            let mut app = state.borrow_mut();
            match event.key().as_str() {
                "ArrowLeft" => app.angle -= 0.12,
                "ArrowRight" => app.angle += 0.12,
                "r" | "Home" => app.pending = Some("key Home".into()),
                _ => return,
            }
            event.prevent_default();
        });
    web_sys::window()
        .ok_or("no window")?
        .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
        .map_err(|e| format!("{e:?}"))?;
    handler.forget();
    Ok(())
}
