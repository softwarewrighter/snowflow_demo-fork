use xetal_world_sweep::Sweep;

#[test]
fn sweep_moves_restarts_and_finishes_with_bounded_geometry() {
    let mut sweep = Sweep::new().unwrap();
    assert!(sweep.tick(0.1).unwrap().brush.is_none());
    sweep.cast([0.0, 8.0], 0.0).unwrap();
    let first = sweep.tick(0.1).unwrap();
    let initial = first.brush.unwrap();
    assert_eq!(initial[0], 1.0);
    assert!(
        first
            .mesh
            .unwrap()
            .positions()
            .iter()
            .all(|p| p.is_finite())
    );
    let mut last = initial;
    let mut triangles = 0;
    for _ in 0..30 {
        let update = sweep.tick(0.1).unwrap();
        if let Some(brush) = update.brush {
            assert!(brush[4] >= last[4]);
            last = brush;
        }
        if let Some(mesh) = update.mesh {
            triangles = mesh.indices().len() / 3;
            assert!(triangles <= 384);
        }
    }
    assert!(last[4] > 12.0 && last[4] < 15.0);
    assert_eq!(triangles, 0);
    assert!(sweep.tick(0.1).unwrap().mesh.is_none());
    sweep.cast([10.0, 5.0], 1.0).unwrap();
    let restarted = sweep.tick(0.1).unwrap().brush.unwrap();
    assert_eq!(restarted[0], 2.0);
    assert_eq!(restarted[1..4], [10.0, 5.0, 1.0]);
    assert!(restarted[4] < last[4]);
    sweep.clear().unwrap();
    assert!(
        sweep
            .tick(0.0)
            .unwrap()
            .mesh
            .unwrap()
            .positions()
            .is_empty()
    );
}

#[test]
fn invalid_input_is_rejected() {
    let mut sweep = Sweep::new().unwrap();
    assert!(sweep.cast([f32::NAN, 0.0], 0.0).is_err());
    assert!(sweep.tick(-0.1).is_err());
    assert!(sweep.tick(f32::INFINITY).is_err());
}
