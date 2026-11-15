use xetal_app_session::Session;
#[test]
fn camera_and_controls_remain_source_owned_without_accidental_reset() {
    let mut app = Session::new().unwrap();
    app.input("4", true).unwrap();
    assert_eq!(app.read().unwrap().navigation[6], 0.0);
    app.input("ArrowRight", true).unwrap();
    assert_eq!(app.read().unwrap().navigation[4], 0.1);
    app.input("w", true).unwrap();
    assert_eq!(app.read().unwrap().navigation[1], 1.0);
    app.blur().unwrap();
    assert_eq!(app.read().unwrap().navigation[1], 0.0);
    for _ in 0..80 {
        app.input("ArrowUp", true).unwrap();
    }
    assert!((app.read().unwrap().navigation[5] - 1.2).abs() < 1e-9);
    for _ in 0..80 {
        app.input("ArrowDown", true).unwrap();
    }
    assert!((app.read().unwrap().navigation[5] + 1.2).abs() < 1e-9);
}
#[test]
fn paused_ribbon_can_release_and_reset_does_not_alias_old_gpu_revisions() {
    let mut app = Session::new().unwrap();
    app.input("2", true).unwrap();
    app.tick(0.1).unwrap();
    app.input("p", true).unwrap();
    app.input("2", false).unwrap();
    assert_eq!(app.read().unwrap().stats["ribbon-mode"], 2.0);
    app.input("p", true).unwrap();
    app.input("4", true).unwrap();
    app.tick(0.1).unwrap();
    let old = app.read().unwrap().resources[0].clone();
    app.input("r", true).unwrap();
    let reset = app.read().unwrap().resources[0].clone();
    assert!(reset.revision > old.revision && reset.mesh.positions().is_empty());
    app.input("4", true).unwrap();
    app.tick(0.1).unwrap();
    let next = app.read().unwrap().resources[0].clone();
    assert!(next.revision > reset.revision && !next.mesh.positions().is_empty());
}
#[test]
fn invalid_host_input_is_rejected_without_stopping_the_session() {
    let mut app = Session::new().unwrap();
    assert!(app.input("x\nkey r", true).is_err());
    assert!(app.tick(f32::NAN).is_err());
    assert!(app.tick(-0.1).is_err());
    assert!(app.respond(1, Some([f32::INFINITY; 3])).is_err());
    app.input("4", true).unwrap();
    app.tick(0.1).unwrap();
    assert_eq!(app.read().unwrap().stats["crystals"], 12.0);
}
