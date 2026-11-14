use xetal_world_stream::World;
fn fill(world: &mut World) -> Vec<f32> {
    let mut positions = Vec::new();
    for _ in 0..1000 {
        if let Some(mesh) = world.step([0.0, 0.0, 0.0], 8_000).unwrap() {
            positions = mesh.positions().to_vec();
        }
        if world.resident() == 25 {
            return positions;
        }
    }
    panic!("world did not load");
}
fn height(mesh: &[f32], x: f32, z: f32) -> f32 {
    mesh.as_chunks::<3>()
        .0
        .iter()
        .find(|p| p[0] == x && p[2] == z)
        .unwrap()[1]
}
#[test]
fn bloom_cuts_a_crater_raises_a_rim_and_coexists_with_sweep() {
    let mut world = World::new().unwrap();
    let before = fill(&mut world);
    world.bloom([1.0, 0.0, 0.0]).unwrap();
    world.sweep([1.0, 16.0, 8.0, 0.0, 13.0]).unwrap();
    let mesh = world.step([0.0, 0.0, 0.0], 8_000).unwrap().unwrap();
    assert!(height(mesh.positions(), 0.0, 0.0) < height(&before, 0.0, 0.0) - 0.8);
    assert!(height(mesh.positions(), 4.0, 0.0) > height(&before, 4.0, 0.0) + 0.2);
    assert!(height(mesh.positions(), 16.0, 0.0) < height(&before, 16.0, 0.0) - 0.5);
    let mut unique = std::collections::BTreeMap::new();
    for p in mesh.positions().as_chunks::<3>().0 {
        if let Some(y) = unique.insert((p[0].to_bits(), p[2].to_bits()), p[1].to_bits()) {
            assert_eq!(y, p[1].to_bits());
        }
    }
    world.clear_edits();
    assert!(
        world
            .step([0.0, 0.0, 0.0], 8_000)
            .unwrap()
            .unwrap()
            .positions()
            == before
    );
}
#[test]
fn fifth_bloom_restores_the_oldest_crater_and_bounds_history() {
    let mut world = World::new().unwrap();
    let before = fill(&mut world);
    let mut last = Vec::new();
    for id in 1..=5 {
        let point = if id == 1 { 0.0 } else { 500.0 };
        world.bloom([id as f32, point, point]).unwrap();
        if let Some(mesh) = world.step([0.0, 0.0, 0.0], 8_000).unwrap() {
            last = mesh.positions().to_vec();
        }
    }
    assert!(last == before);
    assert!(world.bloom([f32::NAN; 3]).is_err());
    assert_eq!(world.resident(), 25);
}
