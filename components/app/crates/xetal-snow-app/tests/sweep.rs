use xetal_snow_app::Scene;
#[test]
fn sweep_changes_the_shared_collision_surface_and_pause_freezes_it() {
    let mut scene = Scene::new().unwrap();
    let mut terrain = Vec::new();
    for _ in 0..1000 {
        let update = scene.update(0.0, [false; 4], false, true).unwrap();
        if let Some(mesh) = update.terrain {
            terrain = mesh.positions().to_vec();
        }
        if scene.world.resident() == 25 {
            break;
        }
    }
    let point = [0.0, 0.0, 0.0];
    let before = xetal_world_navigation::floor(&terrain, point).unwrap();
    scene.sweep.cast([0.0, 8.0], 0.0).unwrap();
    for _ in 0..30 {
        if let Some(mesh) = scene.update(0.1, [false; 4], false, false).unwrap().terrain {
            terrain = mesh.positions().to_vec();
        }
    }
    let floor = xetal_world_navigation::floor(&terrain, point).unwrap();
    assert!(floor < before - 0.5);
    scene.walker.position = [0.0, floor + 2.0, 0.0];
    scene.walker.motion.grounded = false;
    scene.walker.motion.height = floor + 2.0;
    for _ in 0..120 {
        scene.update(1.0 / 60.0, [false; 4], false, false).unwrap();
    }
    assert!((scene.walker.position[1] - floor).abs() < 0.001);
    let revision = scene.world.edit_revision;
    for _ in 0..10 {
        assert!(
            scene
                .update(0.1, [false; 4], false, true)
                .unwrap()
                .sweep
                .is_none()
        );
    }
    assert_eq!(scene.world.edit_revision, revision);
    scene.reset().unwrap();
    let reset = scene.update(0.0, [false; 4], false, true).unwrap();
    assert!(reset.sweep.unwrap().positions().is_empty());
    assert_eq!(
        xetal_world_navigation::floor(reset.terrain.unwrap().positions(), point),
        Some(before)
    );
}
