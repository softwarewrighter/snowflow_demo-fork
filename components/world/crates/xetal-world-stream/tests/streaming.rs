use xetal_world_stream::{World, center, ring};

fn fill(world: &mut World, position: [f32; 3]) {
    for _ in 0..1000 {
        world.step(position, 8_000).unwrap();
        if world.resident() == 25 {
            return;
        }
    }
    panic!("chunk ring did not fill");
}

#[test]
fn streaming_crosses_old_bounds_and_keeps_only_the_nearby_ring() {
    let mut world = World::new().unwrap();
    fill(&mut world, [8.0, 0.0, 8.0]);
    let generated = world.generated;
    for _ in 0..20 {
        assert!(world.step([8.0, 0.0, 8.0], 8_000).unwrap().is_none());
    }
    assert_eq!(
        world.generated, generated,
        "idle must not generate or deform terrain"
    );
    fill(&mut world, [40.0, 0.0, 8.0]);
    assert_eq!(
        world.generated - generated,
        5,
        "one row prefetched across a boundary"
    );
    fill(&mut world, [-1024.0, 0.0, 2048.0]);
    assert_eq!(world.resident(), 25);
    assert_eq!(world.center, (-32, 64));
}

#[test]
fn negative_positions_and_planning_are_deterministic() {
    assert_eq!(center([-0.01, 0.0, -32.01]), (-1, -2));
    let keys = ring((123, -456));
    assert_eq!(keys.len(), 25);
    assert_eq!(keys[0], (123, -456));
    assert_eq!(keys, ring((123, -456)));
}
