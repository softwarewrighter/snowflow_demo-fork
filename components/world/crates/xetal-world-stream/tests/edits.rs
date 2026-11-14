use xetal_world_stream::World;
fn fill(world: &mut World, point: [f32; 3]) -> Vec<f32> {
    let mut mesh = Vec::new();
    for _ in 0..1000 {
        if let Some(next) = world.step(point, 8_000).unwrap() {
            mesh = next.positions().to_vec();
        }
        if world.resident() == 25 {
            return mesh;
        }
    }
    panic!("world did not fill");
}
fn height(mesh: &[f32], x: f32, z: f32) -> f32 {
    mesh.as_chunks::<3>()
        .0
        .iter()
        .find(|p| p[0] == x && p[2] == z)
        .unwrap()[1]
}
#[test]
fn shared_edit_mesh_carves_center_raises_banks_and_clears_exactly() {
    let mut world = World::new().unwrap();
    let original = fill(&mut world, [0.0, 0.0, 8.0]);
    world.sweep([1.0, 0.0, 8.0, 0.0, 13.0]).unwrap();
    let edited = world.step([0.0, 0.0, 8.0], 8_000).unwrap().unwrap();
    assert!(height(edited.positions(), 0.0, 0.0) < height(&original, 0.0, 0.0) - 0.5);
    assert!(height(edited.positions(), 4.0, 0.0) > height(&original, 4.0, 0.0) + 0.1);
    assert!(height(edited.positions(), 0.0, 16.0) == height(&original, 0.0, 16.0));
    assert_eq!(world.resident(), 25);
    world.clear_edits();
    let cleared = world.step([0.0, 0.0, 8.0], 8_000).unwrap().unwrap();
    assert!(
        cleared.positions() == original,
        "reset must restore the exact terrain mesh"
    );
}

#[test]
fn edited_chunk_seams_match_and_revisits_replay_retained_edits() {
    let mut world = World::new().unwrap();
    fill(&mut world, [0.0, 0.0, 8.0]);
    world.sweep([1.0, 0.0, 8.0, 0.0, 13.0]).unwrap();
    let edited = world.step([0.0, 0.0, 8.0], 8_000).unwrap().unwrap();
    let mut vertices = std::collections::BTreeMap::new();
    for p in edited.positions().as_chunks::<3>().0 {
        if let Some(y) = vertices.insert((p[0].to_bits(), p[2].to_bits()), p[1].to_bits()) {
            assert_eq!(
                y,
                p[1].to_bits(),
                "duplicate world coordinates must share an exact height at {p:?}"
            );
        }
    }
    fill(&mut world, [1024.0, 0.0, 1024.0]);
    let revisit = fill(&mut world, [0.0, 0.0, 8.0]);
    assert!(
        edited.positions() == revisit,
        "reloaded chunks must replay retained brush history"
    );
}

#[test]
fn updating_one_cast_replaces_it_and_ninth_cast_evicts_oldest() {
    let mut world = World::new().unwrap();
    let original = fill(&mut world, [0.0, 0.0, 8.0]);
    for reach in [2.0, 4.0, 8.0, 13.0] {
        world.sweep([1.0, 0.0, 8.0, 0.0, reach]).unwrap();
        let mesh = world.step([0.0, 0.0, 8.0], 8_000).unwrap().unwrap();
        let cut = height(mesh.positions(), 0.0, 0.0) - height(&original, 0.0, 0.0);
        assert!(
            cut >= -0.651,
            "one moving cast must not accumulate repeated cuts"
        );
    }
    let mut final_mesh = Vec::new();
    for id in 2..=9 {
        world.sweep([id as f32, 500.0, 500.0, 0.0, 13.0]).unwrap();
        if let Some(mesh) = world.step([0.0, 0.0, 8.0], 8_000).unwrap() {
            final_mesh = mesh.positions().to_vec();
        }
    }
    assert!(
        final_mesh == original,
        "evicting the oldest brush must restore its old resident region"
    );
    assert!(world.sweep([0.0; 5]).is_err());
    assert!(world.sweep([f32::NAN; 5]).is_err());
}
