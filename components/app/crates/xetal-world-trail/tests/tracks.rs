use xetal_world_trail::Trail;
const FLAT: [f32; 9] = [
    -1000.0, 0.0, -1000.0, 1000.0, 0.0, -1000.0, 0.0, 0.0, 1000.0,
];

#[test]
fn idle_and_airborne_movement_do_not_emit_steps_and_history_is_bounded() {
    let mut trail = Trail::new([0.0; 3]);
    for _ in 0..100 {
        assert!(!trail.update([0.0; 3], 0.0, true, 0.0, &FLAT));
    }
    assert!(!trail.update([1.0, 1.0, 0.0], 0.0, false, 0.0, &FLAT));
    for i in 2..400 {
        assert!(trail.update([i as f32, 0.0, 0.0], 0.0, true, 0.0, &FLAT));
    }
    assert_eq!(trail.marks.len(), 256);
    assert!(trail.marks[0].position[0] < trail.marks[255].position[0]);
}

#[test]
fn landing_leaves_two_marks_without_a_walking_crunch() {
    let mut trail = Trail::new([0.0; 3]);
    assert!(!trail.update([0.0; 3], 0.0, true, 5.0, &FLAT));
    assert_eq!(trail.marks.len(), 2);
    assert_eq!(trail.steps, 0);
    assert_eq!(trail.vertices().len(), 60);
    assert!(
        trail
            .vertices()
            .as_chunks::<5>()
            .0
            .iter()
            .all(|v| (v[1] - 0.012).abs() < 0.0001)
    );
}
