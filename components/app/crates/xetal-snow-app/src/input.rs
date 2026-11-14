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
        if let Err(error) = app.scene.reset() {
            status::error(error);
        }
    })?;
    button("clear-tracks", state.clone(), |app| {
        if let Err(error) = app.scene.clear_tracks() {
            status::error(error);
        }
    })?;
    button("start-sound", state.clone(), |app| {
        crate::boot::enter(app, true)
    })?;
    button("start-muted", state.clone(), |app| {
        crate::boot::enter(app, false)
    })?;
    button("sound", state.clone(), crate::boot::sound)?;
    button("crystallise", state.clone(), |app| cast(app, false))?;
    button("sweep", state.clone(), |app| cast(app, true))?;
    button("textures", state.clone(), |app| {
        app.effects[0] = !app.effects[0];
        status::text(
            "textures",
            if app.effects[0] {
                "Textures: on"
            } else {
                "Textures: off"
            },
        );
    })?;
    button("shadows", state.clone(), |app| {
        app.effects[1] = !app.effects[1];
        status::text(
            "shadows",
            if app.effects[1] {
                "Shadows: on"
            } else {
                "Shadows: off"
            },
        );
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
            let target = event
                .target()
                .and_then(|node| node.dyn_into::<web_sys::Element>().ok());
            if target.is_some_and(|node| {
                node.tag_name() == "PRE" || (node.tag_name() == "BUTTON" && event.key() == " ")
            }) {
                return;
            }
            if (event.key() == " " || event.key() == "4" || event.key() == "1") && event.repeat() {
                event.prevent_default();
                return;
            }
            let mut app = state.borrow_mut();
            if !app.started {
                return;
            }
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
        " " if pressed => app.jump = true,
        "1" | "4" if pressed => cast(app, key == "1"),
        "w" | "W" => app.held[0] = pressed,
        "s" | "S" => app.held[1] = pressed,
        "a" | "A" => app.held[2] = pressed,
        "d" | "D" => app.held[3] = pressed,
        "ArrowLeft" if pressed => app.scene.walker.yaw -= 0.10,
        "ArrowRight" if pressed => app.scene.walker.yaw += 0.10,
        "ArrowUp" if pressed => app.scene.walker.pitch = (app.scene.walker.pitch + 0.06).min(1.2),
        "ArrowDown" if pressed => {
            app.scene.walker.pitch = (app.scene.walker.pitch - 0.06).max(-1.2)
        }
        "r" | "Home" if pressed => {
            if let Err(error) = app.scene.reset() {
                status::error(error);
            }
        }
        _ => return false,
    }
    true
}

fn cast(app: &mut App, sweep: bool) {
    if !app.started || app.paused {
        return;
    }
    let walker = &app.scene.walker;
    let point = [walker.position[0], walker.position[2]];
    let result = if sweep {
        app.scene.sweep.cast(point, walker.yaw)
    } else {
        app.scene.spells.cast(point, walker.yaw)
    };
    if let Err(error) = result {
        status::error(error);
    }
}
