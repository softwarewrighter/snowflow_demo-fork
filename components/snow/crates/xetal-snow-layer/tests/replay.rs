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
