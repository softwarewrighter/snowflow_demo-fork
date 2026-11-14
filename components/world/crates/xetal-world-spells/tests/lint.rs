#[test]
fn spell_source_has_no_enabled_lint_findings() {
    let findings = xetal_linter::lint(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../demos/crystallise.xtl"),
        xetal_world_spells::SOURCE,
        &[],
    )
    .unwrap();
    assert!(findings.is_empty(), "{findings:#?}");
}

#[test]
fn rendered_spell_links_to_its_actual_library() {
    let html = xetal_world_model::highlighted_text(xetal_world_spells::SOURCE);
    assert!(html.contains("library-reference.html#crystallise"));
    assert!(html.contains("c-libfunc"));
}
