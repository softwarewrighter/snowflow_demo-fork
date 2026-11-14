use xetal_world_trail::Trail;
const FLAT: [f32; 9] = [
    -1000.0, 0.0, -1000.0, 1000.0, 0.0, -1000.0, 0.0, 0.0, 1000.0,
];
#[test]
fn backend_obeys_xetal_opacity_and_reuses_draped_geometry() {
    let mut trail = Trail::default();
    trail
        .replace(&[[0.0, 0.0, 0.0, 1.0, 0.0, 1.0]], &FLAT)
        .unwrap();
    let geometry = trail.marks[0].vertices;
    trail
        .replace(&[[0.0, 0.0, 0.0, 1.0, 0.0, 0.25]], &[])
        .unwrap();
    assert_eq!(trail.marks[0].vertices, geometry);
    assert_eq!(trail.marks[0].opacity, 0.25);
    assert_eq!(trail.vertices().len(), 36);
    assert!(
        trail
            .vertices()
            .as_chunks::<6>()
            .0
            .iter()
            .all(|v| v[5] == 0.25)
    );
    trail.replace(&[], &FLAT).unwrap();
    assert!(trail.vertices().is_empty());
}
#[test]
fn backend_rejects_over_budget_commands() {
    assert!(Trail::default().replace(&[[0.0; 6]; 257], &FLAT).is_err());
}
