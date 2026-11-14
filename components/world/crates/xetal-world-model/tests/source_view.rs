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
