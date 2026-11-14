#[test]
fn bloom_program_lints_and_links_its_real_library() {
    let source = xetal_world_bloom::SOURCE;
    let findings = xetal_linter::lint(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../world/demos/bloom.xtl"
        ),
        source,
        &[],
    )
    .unwrap();
    assert!(findings.is_empty(), "{findings:#?}");
    assert!(xetal_world_model::highlighted_text(source).contains("library-reference.html#bloom"));
}
