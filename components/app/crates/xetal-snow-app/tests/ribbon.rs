use xetal_snow_app::Scene;
fn ready() -> Scene {
    let mut scene = Scene::new().unwrap();
    for _ in 0..1000 {
        scene.update(0.0, [false; 4], false, true).unwrap();
        if scene.world.resident() == 25 {
            return scene;
        }
    }
    panic!("scene did not load");
}
#[test]
fn held_ribbon_tracks_pose_pauses_and_clears_on_reset() {
    let mut scene = ready();
    scene.ribbon_input(true).unwrap();
    for _ in 0..15 {
        scene.update(0.05, [false; 4], false, false).unwrap();
    }
    assert_eq!(scene.ribbon.mode(), 1);
    assert!(!scene.ribbon.positions().is_empty());
    let old = scene.ribbon.positions().to_vec();
    scene.update(0.1, [false; 4], false, true).unwrap();
    assert!(old == scene.ribbon.positions());
    scene.ribbon_input(false).unwrap();
    for _ in 0..80 {
        scene.update(0.05, [false; 4], false, false).unwrap();
    }
    assert_eq!(scene.ribbon.mode(), 0);
    assert!(scene.ribbon.positions().is_empty());
    scene.ribbon_input(true).unwrap();
    scene.update(0.05, [false; 4], false, false).unwrap();
    scene.reset().unwrap();
    assert!(
        scene
            .update(0.0, [false; 4], false, true)
            .unwrap()
            .ribbon
            .unwrap()
            .positions()
            .is_empty()
    );
}
