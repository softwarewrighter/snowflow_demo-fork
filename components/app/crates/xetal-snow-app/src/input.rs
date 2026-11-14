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
        app.walker = Default::default();
        app.pending = Some("key Home".into())
    })?;
    button("raise", state.clone(), |app| {
        app.pending = Some("down 0 0".into())
    })?;
    button("scene", state.clone(), |app| {
        app.pending = Some("down 0 0".into())
    })?;
    keys(state.clone(), true)?;
    keys(state.clone(), false)?;
    blur(state)
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

fn keys(state: State, pressed: bool) -> Result<(), String> {
    let handler =
        Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            let mut app = state.borrow_mut();
            if !command(&mut app, &event.key(), pressed) {
                return;
            }
            event.prevent_default();
        });
    web_sys::window()
        .ok_or("no window")?
        .add_event_listener_with_callback(
            if pressed { "keydown" } else { "keyup" },
            handler.as_ref().unchecked_ref(),
        )
        .map_err(|e| format!("{e:?}"))?;
    handler.forget();
    Ok(())
}

fn blur(state: State) -> Result<(), String> {
    let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        state.borrow_mut().held = [false; 4];
    });
    web_sys::window()
        .ok_or("no window")?
        .add_event_listener_with_callback("blur", handler.as_ref().unchecked_ref())
        .map_err(|e| format!("{e:?}"))?;
    handler.forget();
    Ok(())
}

fn command(app: &mut App, key: &str, pressed: bool) -> bool {
    match key {
        "w" | "W" => app.held[0] = pressed,
        "s" | "S" => app.held[1] = pressed,
        "a" | "A" => app.held[2] = pressed,
        "d" | "D" => app.held[3] = pressed,
        "ArrowLeft" if pressed => app.walker.yaw -= 0.10,
        "ArrowRight" if pressed => app.walker.yaw += 0.10,
        "ArrowUp" if pressed => app.walker.pitch = (app.walker.pitch + 0.06).min(1.2),
        "ArrowDown" if pressed => app.walker.pitch = (app.walker.pitch - 0.06).max(-1.2),
        "r" | "Home" if pressed => {
            app.walker = Default::default();
            app.pending = Some("key Home".into());
        }
        _ => return false,
    }
    true
}
