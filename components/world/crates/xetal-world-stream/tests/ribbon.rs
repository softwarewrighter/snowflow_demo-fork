use xetal_world_stream::World;
fn fill(world: &mut World) -> Vec<f32> {
    let mut positions = Vec::new();
    for _ in 0..1000 {
        if let Some(mesh) = world.step([0.0; 3], 8_000).unwrap() {
            positions = mesh.positions().to_vec();
        }
        if world.resident() == 25 {
            return positions;
        }
    }
    panic!("terrain did not fill");
}
fn height(mesh: &[f32], x: f32, z: f32) -> f32 {
    mesh.as_chunks::<3>()
        .0
        .iter()
        .find(|p| p[0] == x && p[2] == z)
        .unwrap()[1]
}
#[test]
fn ribbon_scores_shared_terrain_and_evicts_its_oldest_stamp() {
    let mut world = World::new().unwrap();
    let original = fill(&mut world);
    world.ribbon([1.0, 0.0, 0.0, 1.6]).unwrap();
    let edited = world.step([0.0; 3], 8_000).unwrap().unwrap();
    assert!(
        (height(edited.positions(), 0.0, 0.0) - height(&original, 0.0, 0.0) + 0.22).abs() < 0.001
    );
    let mut unique = std::collections::BTreeMap::new();
    for p in edited.positions().as_chunks::<3>().0 {
        if let Some(y) = unique.insert((p[0].to_bits(), p[2].to_bits()), p[1].to_bits()) {
            assert_eq!(y, p[1].to_bits());
        }
    }
    let mut restored = Vec::new();
    for id in 2..=17 {
        world.ribbon([id as f32, 500.0, 500.0, 1.6]).unwrap();
        if let Some(mesh) = world.step([0.0; 3], 8_000).unwrap() {
            restored = mesh.positions().to_vec();
        }
    }
    assert!(restored == original);
    assert!(world.ribbon([f32::NAN; 4]).is_err());
}
