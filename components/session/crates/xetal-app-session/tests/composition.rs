use xetal_app_session::{Pose, Session};
fn pose() -> Pose {
    Pose {
        position: [0.0, 4.0, 8.0],
        aim: [0.0, -0.12],
        grounded: true,
        impact: 0.0,
    }
}
#[test]
fn xetal_configures_buttons_and_dispatches_actions() {
    let mut app = Session::new().unwrap();
    app.pose(pose()).unwrap();
    let initial = app.read().unwrap();
    assert_eq!(
        initial
            .buttons
            .iter()
            .take(4)
            .map(|b| b.key.as_str())
            .collect::<Vec<_>>(),
        ["1", "2", "3", "4"]
    );
    app.input("4", true).unwrap();
    app.tick(0.1).unwrap();
    let frame = app.read().unwrap();

    assert_eq!(frame.stats["crystals"], 12.0);
    assert!(
        frame
            .resources
            .iter()
            .any(|r| r.material == "ice" && !r.mesh.positions().is_empty())
    );
    app.input("2", true).unwrap();
    for _ in 0..10 {
        app.tick(0.05).unwrap();
    }
    assert_eq!(app.read().unwrap().stats["ribbon-mode"], 1.0);
    app.input("2", false).unwrap();
    app.tick(0.1).unwrap();
    assert_eq!(app.read().unwrap().stats["ribbon-mode"], 2.0);
}
#[test]
fn changing_xetal_bindings_changes_real_dispatch_without_rust_changes() {
    let source = xetal_app_session::SOURCE.replace(
        "bindings := \"1\" \"2\" \"3\" \"4\"",
        "bindings := \"1\" \"2\" \"3\" \"9\"",
    );
    let mut app = Session::from_source(&source).unwrap();
    app.pose(pose()).unwrap();
    app.input("4", true).unwrap();
    assert_eq!(app.read().unwrap().stats["crystals"], 0.0);
    app.input("9", true).unwrap();
    app.tick(0.1).unwrap();
    assert_eq!(app.read().unwrap().stats["crystals"], 12.0);
}
