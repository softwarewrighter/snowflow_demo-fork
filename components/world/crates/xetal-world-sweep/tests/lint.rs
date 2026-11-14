#[test]
fn sweep_and_terrain_programs_lint_and_link_to_actual_libraries() {
    let source = xetal_world_sweep::SOURCE;
    let findings = xetal_linter::lint(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../demos/sweep.xtl"),
        source,
        &[],
    )
    .unwrap();
    assert!(findings.is_empty(), "{findings:#?}");
    let html = xetal_world_model::highlighted_text(source);
    assert!(html.contains("library-reference.html#sweep"));
}
