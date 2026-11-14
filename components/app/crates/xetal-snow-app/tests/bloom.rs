use xetal_snow_app::Scene;
#[test]
fn bloom_targets_loaded_ground_updates_once_and_reset_clears_particles_and_crater() {
    let mut scene = Scene::new().unwrap();
    let mut original = 0;
    for _ in 0..1000 {
        scene.update(0.0, [false; 4], false, true).unwrap();
        if scene.world.resident() == 25 {
            original = scene.mesh_hash;
            break;
        }
    }
    scene.cast_bloom().unwrap();
    let first = scene.update(0.1, [false; 4], false, false).unwrap();
    assert!(first.bloom.is_some() && first.particles.is_some());
    assert_ne!(scene.mesh_hash, original);
    let revision = scene.world.edit_revision;
    for _ in 0..10 {
        scene.update(0.1, [false; 4], false, false).unwrap();
    }
    assert_eq!(revision, scene.world.edit_revision);
    let frozen = scene.bloom.particles().to_vec();
    let paused = scene.update(0.1, [false; 4], false, true).unwrap();
    assert!(paused.bloom.is_none() && paused.particles.is_none());
    assert_eq!(frozen, scene.bloom.particles());
    scene.reset().unwrap();
    let reset = scene.update(0.0, [false; 4], false, true).unwrap();
    assert!(reset.bloom.unwrap().positions().is_empty());
    assert!(reset.particles.unwrap().is_empty());
    assert_eq!(scene.mesh_hash, original);
}
