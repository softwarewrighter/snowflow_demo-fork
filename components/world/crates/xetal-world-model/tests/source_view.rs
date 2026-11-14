use xetal_world_model::{SOURCE, rendered_source};

#[test]
fn shipped_program_uses_language_decoration_and_preserves_lines_and_strings() {
    let rendered = rendered_source().unwrap();
    assert_ne!(rendered, SOURCE);
    assert_eq!(rendered.lines().count(), SOURCE.lines().count());
    assert!(rendered.contains("phase! \u{2190} 0.0"));
    for literal in ["\"tick\"", "\"down\"", "\"key\"", "\"end\""] {
        assert!(rendered.contains(literal));
    }
    assert_eq!(rendered, xetal_render::decorate(SOURCE).unwrap());
}

#[test]
fn highlighted_source_has_semantic_colors_and_escapes_text() {
    let html = xetal_world_model::highlighted_source();
    for class in ["c-builtin", "c-userfunc", "c-number", "c-string", "c-doc"] {
        assert!(html.contains(&format!("class=\"{class}\"")), "{class}");
    }
    assert!(html.contains("&quot;tick&quot;"));
    let malicious = xetal_view::html(&xetal_view::view("\"<script>&\""));
    assert!(malicious.contains("&lt;script&gt;&amp;"));
    assert!(!malicious.contains("<script>"));
}
