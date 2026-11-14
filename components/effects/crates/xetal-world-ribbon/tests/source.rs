#[test]
fn ribbon_source_lints_and_links_actual_libraries() {
    let source = xetal_world_ribbon::SOURCE;
    let findings = xetal_linter::lint(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../world/demos/ribbon.xtl"
        ),
        source,
        &[],
    )
    .unwrap();
    assert!(findings.is_empty(), "{findings:#?}");
    assert!(xetal_world_model::highlighted_text(source).contains("library-reference.html#ribbon"));
}
