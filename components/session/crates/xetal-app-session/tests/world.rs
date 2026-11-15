use std::sync::Arc;
use xetal_app_session::Session;
#[test]
fn dirty_regions_accumulate_across_spells_until_current_version_acknowledged() {
    let mut app = Session::new().unwrap();
    app.input("1", true).unwrap();
    app.tick(0.1).unwrap();
    let first = app.read().unwrap();
    assert!(!first.dirty.is_empty());
    app.input("3", true).unwrap();
    app.respond(1, Some([20.0, 0.0, 20.0])).unwrap();
    app.tick(0.1).unwrap();
    let both = app.read().unwrap();
    assert!(both.world_revision > first.world_revision);
    assert!(both.dirty.len() > first.dirty.len());
    app.chunk((0, 0)).unwrap();
    app.acknowledge(first.world_revision).unwrap();
    assert_eq!(app.read().unwrap().dirty, both.dirty);
    app.acknowledge(both.world_revision).unwrap();
    assert!(app.read().unwrap().dirty.is_empty());
}
#[test]
fn settled_geometry_reuses_native_resources_and_reset_restores_original_terrain() {
    let mut app = Session::new().unwrap();
    let original = app.chunk((0, 0)).unwrap().positions().to_vec();
    app.input("4", true).unwrap();
    for _ in 0..25 {
        app.tick(0.1).unwrap();
    }
    let before = app.read().unwrap().resources[0].clone();
    app.tick(0.1).unwrap();
    let after = app.read().unwrap().resources[0].clone();
    assert_eq!(before.revision, after.revision);
    assert!(Arc::ptr_eq(&before.mesh, &after.mesh));
    app.input("1", true).unwrap();
    for _ in 0..5 {
        app.tick(0.1).unwrap();
    }
    assert!(app.chunk((0, 0)).unwrap().positions() != original);
    app.input("r", true).unwrap();
    assert_eq!(app.chunk((0, 0)).unwrap().positions(), original);
}
