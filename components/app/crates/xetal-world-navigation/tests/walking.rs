use xetal_world_navigation::Walker;

#[test]
fn movement_is_unbounded_and_independent_of_frame_rate() {
    let mut slow = Walker::default();
    let mut fast = Walker::default();
    for _ in 0..10 {
        slow.advance(0.1, 1.0, 1.0);
    }
    for _ in 0..100 {
        fast.advance(0.01, 1.0, 1.0);
    }
    assert!((slow.position[0] - fast.position[0]).abs() < 0.001);
    assert!((slow.position[2] - fast.position[2]).abs() < 0.001);
    for _ in 0..1000 {
        slow.advance(0.1, 1.0, 0.0);
    }
    assert!(slow.position[2] < -300.0);
    slow = Walker::default();
    assert_eq!(slow.position, [0.0, 1.7, 8.0]);
}

#[test]
fn ground_uses_triangle_interpolation_and_preserves_eye_clearance() {
    let mut walker = Walker::default();
    let triangle = [-20.0, 0.0, -20.0, 20.0, 4.0, -20.0, 0.0, 2.0, 20.0];
    walker.ground(&triangle);
    assert!((walker.position[1] - 3.7).abs() < 0.0001);
    walker.position[0] = 100.0;
    walker.ground(&triangle);
    assert!((walker.position[1] - 3.7).abs() < 0.0001);
}
