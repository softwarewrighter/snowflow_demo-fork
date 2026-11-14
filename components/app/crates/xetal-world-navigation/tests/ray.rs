use xetal_world_navigation::raycast;
#[test]
fn raycast_returns_nearest_ground_hit_and_rejects_out_of_range_or_invalid_rays() {
    let mesh = [-10.0, 0.0, -10.0, 10.0, 0.0, -10.0, 0.0, 0.0, 10.0];
    assert_eq!(
        raycast(&mesh, [0.0, 2.0, 0.0], [0.0, -1.0, 0.0], 24.0),
        Some([0.0, 0.0, 0.0])
    );
    assert!(raycast(&mesh, [0.0, 2.0, 0.0], [0.0, 1.0, 0.0], 24.0).is_none());
    assert!(raycast(&mesh, [0.0, 2.0, 0.0], [0.0, -1.0, 0.0], 1.0).is_none());
    assert!(raycast(&mesh, [f32::NAN; 3], [0.0, -1.0, 0.0], 24.0).is_none());
}
