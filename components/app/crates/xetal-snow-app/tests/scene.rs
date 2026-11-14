use xetal_snow_app::Scene;

fn ready() -> Scene {
    let mut scene = Scene::new().unwrap();
    for _ in 0..1000 {
        scene.update(1.0 / 60.0, [false; 4], false, true).unwrap();
        if scene.world.resident() == 25 {
            return scene;
        }
    }
    panic!("scene did not load");
}

#[test]
fn resting_ground_and_camera_are_stable_and_emit_no_footfalls() {
    let mut scene = ready();
    let position = scene.walker.position;
    let hash = scene.mesh_hash;
    let generated = scene.world.generated;
    for _ in 0..120 {
        let update = scene.update(1.0 / 60.0, [false; 4], false, false).unwrap();
        assert!(update.terrain.is_none() && !update.step && update.landing == 0.0);
        assert_eq!(scene.walker.position, position);
        assert_eq!(scene.mesh_hash, hash);
    }
    assert_eq!(scene.trail.marks.len(), 0);
    assert_eq!(scene.world.generated, generated);
}

#[test]
fn walking_crosses_original_wall_and_jump_emits_one_landing_event() {
    let mut scene = ready();
    for _ in 0..100 {
        scene
            .update(0.1, [true, false, false, false], false, false)
            .unwrap();
    }
    assert!(scene.walker.position[2] < -15.0);
    assert!(scene.steps > 0 && scene.trail.marks.len() <= 256);
    for _ in 0..120 {
        scene.update(1.0 / 60.0, [false; 4], false, false).unwrap();
    }
    let before = scene.landings;
    scene.clear_tracks().unwrap();
    scene.update(1.0 / 60.0, [false; 4], true, false).unwrap();
    for _ in 0..120 {
        scene.update(1.0 / 60.0, [false; 4], false, false).unwrap();
    }
    assert_eq!(scene.landings - before, 1);
    assert_eq!(scene.steps, 0);
    assert_eq!(scene.trail.marks.len(), 2);
}
