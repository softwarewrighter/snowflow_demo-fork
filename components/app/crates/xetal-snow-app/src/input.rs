use crate::{frame::App, status};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};

type State = Rc<RefCell<App>>;

pub(crate) fn install(state: State) -> Result<(), String> {
    game_buttons(state.clone())?;
    button("start-sound", state.clone(), |app| {
        crate::boot::enter(app, true)
    })?;
    button("start-muted", state.clone(), |app| {
        crate::boot::enter(app, false)
    })?;
    button("sound", state.clone(), crate::boot::sound)?;
    effect_buttons(state.clone())?;
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
            if event.repeat() {
                return;
            }
            let mut app = state.borrow_mut();
            if !app.started {
                return;
            }
            if app.scene.app.input(&event.key(), pressed).is_err() {
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
        let mut app = state.borrow_mut();
        if let Err(error) = app.scene.app.blur() {
            status::error(error);
        }
    });
    web_sys::window()
        .ok_or("no window")?
        .add_event_listener_with_callback("blur", handler.as_ref().unchecked_ref())
        .map_err(|e| format!("{e:?}"))?;
    handler.forget();
    Ok(())
}

fn game_buttons(state: State) -> Result<(), String> {
    let buttons = state.borrow().scene.frame.buttons.clone();
    for spec in buttons {
        let state = state.clone();
        let node = status::element(&spec.id)?;
        let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
            let mut app = state.borrow_mut();
            if !app.started {
                return;
            }
            let active = app
                .scene
                .frame
                .buttons
                .iter()
                .find(|b| b.id == spec.id)
                .is_some_and(|b| b.active);
            if let Err(error) = app.scene.input(&spec.key, !spec.held || !active) {
                status::error(error);
            }
        });
        node.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| format!("{e:?}"))?;
        handler.forget();
    }
    Ok(())
}

fn effect_buttons(state: State) -> Result<(), String> {
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
    Ok(())
}
