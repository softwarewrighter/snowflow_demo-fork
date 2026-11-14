#[test]
fn spell_source_has_no_enabled_lint_findings() {
    let findings = xetal_linter::lint(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../demos/crystallise.xtl"),
        xetal_world_spells::SOURCE, &[],
    ).unwrap();
    assert!(findings.is_empty(), "{findings:#?}");
}
