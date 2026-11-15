use xetal_app_session::{Pose, Session};
use xetal_snow_layer::Layer;
#[test]
fn geometric_boot_field_matches_collision_replay_and_clear() {
    let mut app = Session::new().unwrap();
    let mut layer = Layer::default();
    let base = app.chunk((0, 0)).unwrap();
    let frame = app.read().unwrap();
    layer
        .update(&mut app, &frame, [2.0, 4.0, 7.0], Some(base))
        .unwrap();
    let original = layer.floor([1.8, 0.0, 7.0]).unwrap();
    app.pose(Pose {
        position: [1.95, 4.0, 8.0],
        aim: [0.0, 0.0],
        grounded: true,
        impact: 0.0,
    })
    .unwrap();
    app.pose(Pose {
        position: [1.95, 4.0, 7.0],
        aim: [0.0, 0.0],
        grounded: true,
        impact: 0.0,
    })
    .unwrap();
    let frame = app.read().unwrap();
    let mut mesh = None;
    for _ in 0..10 {
        mesh = layer
            .update(&mut app, &frame, [2.0, 4.0, 7.0], None)
            .unwrap()
            .or(mesh);
    }
    assert!(layer.resident() > 1 && layer.resident() <= 36);
    assert!(layer.memory_bytes() < 2 * 1024 * 1024);
    let mesh = mesh.unwrap();
    let triangles = mesh
        .indices()
        .iter()
        .flat_map(|i| {
            mesh.positions()[*i as usize * 3..*i as usize * 3 + 3]
                .iter()
                .copied()
        })
        .collect::<Vec<_>>();
    for x in [1.9, 1.95, 2.0, 2.05] {
        let point = [x, 0.0, 7.0];
        let expected = xetal_world_navigation::floor(&triangles, point).unwrap();
        assert!((layer.floor(point).unwrap() - expected).abs() < 0.00001);
    }
    assert!(layer.floor([1.8, 0.0, 7.0]).unwrap() < original - 0.01);
    app.input("c", true).unwrap();
    let frame = app.read().unwrap();
    layer
        .update(&mut app, &frame, [2.0, 4.0, 7.0], None)
        .unwrap();
    assert_eq!(layer.floor([1.8, 0.0, 7.0]).unwrap(), original);
}
#[test]
fn pending_tiles_do_not_change_collision_and_clear_cancels_stale_work() {
    let mut app = Session::new().unwrap();
    let mut layer = Layer::default();
    let base = app.chunk((0, 0)).unwrap();
    let frame = app.read().unwrap();
    layer
        .update(&mut app, &frame, [2.0, 4.0, 7.0], Some(base))
        .unwrap();
    let revision = layer.revision;
    let ground = layer.floor([1.8, 0.0, 7.0]);
    for z in [8.0, 7.0] {
        app.pose(Pose {
            position: [1.95, 4.0, z],
            aim: [0.0, 0.0],
            grounded: true,
            impact: 0.0,
        })
        .unwrap();
    }
    let frame = app.read().unwrap();
    assert!(
        layer
            .update(&mut app, &frame, [2.0, 4.0, 7.0], None)
            .unwrap()
            .is_none()
    );
    assert_eq!(layer.revision, revision);
    assert_eq!(layer.floor([1.8, 0.0, 7.0]), ground);
    app.input("c", true).unwrap();
    let frame = app.read().unwrap();
    layer
        .update(&mut app, &frame, [2.0, 4.0, 7.0], None)
        .unwrap();
    assert_eq!(layer.resident(), 0);
    assert_eq!(layer.floor([1.8, 0.0, 7.0]), ground);
}
#[test]
fn higher_resolution_replay_and_eviction_keep_sampling_repeatable() {
    let mut app = Session::new().unwrap();
    let mut layer = Layer::default();
    for z in [8.0, 7.0] {
        app.pose(Pose {
            position: [1.95, 4.0, z],
            aim: [0.0, 0.0],
            grounded: true,
            impact: 0.0,
        })
        .unwrap();
    }
    let base = app.chunk((0, 0)).unwrap();
    let mut frame = app.read().unwrap();
    frame.detail.config[0] = 0.0625;
    layer
        .update(&mut app, &frame, [2.0, 4.0, 7.0], Some(base))
        .unwrap();
    for _ in 0..8 {
        layer
            .update(&mut app, &frame, [2.0, 4.0, 7.0], None)
            .unwrap();
    }
    let original = layer.floor([1.8, 0.0, 7.0]);
    assert!(layer.resident() > 0);
    layer
        .update(&mut app, &frame, [100.0, 4.0, 100.0], None)
        .unwrap();
    assert_eq!(layer.resident(), 0);
    for _ in 0..8 {
        layer
            .update(&mut app, &frame, [2.0, 4.0, 7.0], None)
            .unwrap();
    }
    assert_eq!(layer.floor([1.8, 0.0, 7.0]), original);
    assert!(layer.memory_bytes() < 2 * 1024 * 1024);
}
#[test]
fn outer_detail_boundary_stitches_to_the_coarse_surface() {
    let mut app = Session::new().unwrap();
    let mut layer = Layer::default();
    for z in [8.0, 7.0] {
        app.pose(Pose {
            position: [1.95, 4.0, z],
            aim: [0.0, 0.0],
            grounded: true,
            impact: 0.0,
        })
        .unwrap();
    }
    let base = app.chunk((0, 0)).unwrap();
    let expected = xetal_world_navigation::floor(base.positions(), [2.0, 0.0, 7.9]).unwrap();
    let mut frame = app.read().unwrap();
    frame.detail.config[3] = 1.0;
    let mesh = layer
        .update(&mut app, &frame, [1.8, 4.0, 7.0], Some(base))
        .unwrap()
        .unwrap();
    let triangles = mesh
        .indices()
        .iter()
        .flat_map(|i| {
            mesh.positions()[*i as usize * 3..*i as usize * 3 + 3]
                .iter()
                .copied()
        })
        .collect::<Vec<_>>();
    let rendered = xetal_world_navigation::floor(&triangles, [2.0, 0.0, 7.9]).unwrap();
    assert!((rendered - expected).abs() < 0.00001);
    assert!((layer.floor([2.0, 0.0, 7.9]).unwrap() - rendered).abs() < 0.00001);
}
