use xetal_snow_app::Scene;
#[test]
fn bloom_targets_loaded_ground_updates_once_and_reset_clears_particles_and_crater() {
    let mut scene = Scene::new().unwrap();
    let mut original = 0;
    for _ in 0..1000 {
        scene.update(0.0, true).unwrap();
        if scene.world.resident() == 25 {
            original = scene.mesh_hash;
            break;
        }
    }
    scene.input("3", true).unwrap();
    let first = scene.update(0.1, false).unwrap();
    assert!(!first.resources.is_empty() && first.particles.is_some());
    assert_ne!(scene.mesh_hash, original);
    let revision = scene.world.edit_revision;
    for _ in 0..10 {
        scene.update(0.1, false).unwrap();
    }
    assert_eq!(revision, scene.world.edit_revision);
    let frozen = scene.frame.particles.as_slice().to_vec();
    let paused = scene.update(0.1, true).unwrap();
    assert!(paused.resources.is_empty() && paused.particles.is_none());
    assert_eq!(frozen, scene.frame.particles.as_slice());
    scene.input("r", true).unwrap();
    let reset = scene.update(0.0, true).unwrap();
    assert!(
        reset
            .resources
            .iter()
            .find(|r| r.material == "slush")
            .unwrap()
            .mesh
            .positions()
            .is_empty()
    );
    assert!(reset.particles.unwrap().is_empty());
    assert_eq!(scene.mesh_hash, original);
}
