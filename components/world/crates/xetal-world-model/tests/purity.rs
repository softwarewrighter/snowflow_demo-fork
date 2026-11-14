use xetal_core::{Expr, Item, Kind};
use xetal_world_model::SOURCE;

const BEFORE: &str = include_str!("fixtures/snowfield-mutable.xtl");

fn assignments(expr: &Expr) -> usize {
    if let Kind::Set { name, .. } = &expr.kind {
        assert_eq!(name, "frame!");
    }
    if let Kind::Let { name, .. } = &expr.kind {
        assert!(!name.ends_with('!'));
    }
    usize::from(matches!(expr.kind, Kind::Set { .. }))
        + expr.children().into_iter().map(assignments).sum::<usize>()
}

#[test]
fn only_the_effect_loop_may_publish_to_the_mesh_slot() {
    let loaded =
        xetal_program::load_with("snowfield.xtl", SOURCE, &xetal_snow_libraries::Embedded).unwrap();
    let program = loaded.program;
    let mut mutable = Vec::new();
    let mut publications = 0;
    for item in &program.items {
        match item {
            Item::Let { name, .. } if name.ends_with('!') => mutable.push(name.as_str()),
            Item::Def { name, value } if name == "u:l_oop!" => publications += assignments(value),
            Item::Def { value, .. } | Item::Let { value, .. } | Item::Eval(value) => {
                assert_eq!(assignments(value), 0)
            }
            Item::Set { .. } => panic!("no top-level reassignment allowed"),
        }
    }
    assert_eq!(mutable, ["frame!"]);
    assert_eq!(publications, 1);
}

#[test]
fn shipped_xetal_source_has_no_enabled_lint_findings() {
    let findings = xetal_linter::lint(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../demos/snowfield.xtl"),
        SOURCE,
        &[],
    )
    .unwrap();
    assert!(findings.is_empty(), "{findings:#?}");
}

#[test]
fn lint_driver_catches_the_legacy_unused_effect_binding() {
    let findings = xetal_linter::lint("before.xtl", BEFORE, &[]).unwrap();
    assert!(
        findings
            .iter()
            .any(|finding| finding.lint.name == "unused-binding")
    );
}
