pub(crate) fn element(id: &str) -> Result<web_sys::Element, String> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
        .ok_or_else(|| format!("missing page element: {id}"))
}

pub(crate) fn text(id: &str, text: &str) {
    if let Ok(node) = element(id) {
        node.set_text_content(Some(text));
    }
}

pub(crate) fn error(error: String) {
    text("status", &error);
    text("splash-status", &error);
    if let Ok(splash) = element("splash") {
        let _ = splash.set_attribute("hidden", "");
    }
    if let Ok(node) = element("app") {
        let _ = node.set_attribute("data-state", "error");
    }
    web_sys::console::error_1(&error.into());
}

pub(crate) fn progress(frames: u32, updates: u32, mesh_hash: u32) {
    text("status", "Live · X_eTaL arrays → Rust → WebGPU");
    text("updates", &format!("{updates} mesh updates"));
    if let Ok(node) = element("app") {
        let _ = node.set_attribute("data-state", "ready");
        let _ = node.set_attribute("data-frames", &frames.to_string());
        let _ = node.set_attribute("data-updates", &updates.to_string());
        let _ = node.set_attribute("data-mesh", &mesh_hash.to_string());
    }
}

pub(crate) fn failed() -> bool {
    element("app")
        .ok()
        .and_then(|node| node.get_attribute("data-state"))
        .as_deref()
        == Some("error")
}
