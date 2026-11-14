//! Browser composition for the X_eTaL procedural surface demo.
pub use xetal_world_model::SOURCE;
#[cfg(target_arch = "wasm32")]
mod boot;
#[cfg(target_arch = "wasm32")]
mod frame;
#[cfg(target_arch = "wasm32")]
mod input;
#[cfg(target_arch = "wasm32")]
mod status;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = boot::start().await {
            status::error(error);
        }
    });
}

mod effects;
mod scene;
pub use scene::Scene;
