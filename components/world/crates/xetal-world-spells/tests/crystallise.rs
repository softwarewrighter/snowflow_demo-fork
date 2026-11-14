use xetal_world_spells::Spells;

#[test]
fn crystals_grow_deterministically_and_stay_bounded() {
    let mut a = Spells::new().unwrap();
    let mut b = Spells::new().unwrap();
    assert!(a.tick(0.1).unwrap().is_none());
    for spell in [&mut a, &mut b] {
        spell.cast([0.0, 8.0], 0.0).unwrap();
    }
    let early = a.tick(0.1).unwrap().unwrap();
    b.tick(0.1).unwrap();
    let early_positions = early.positions().to_vec();
    let mut grown = early;
    for _ in 0..20 {
        if let Some(mesh) = a.tick(0.1).unwrap() { grown = mesh; }
        b.tick(0.1).unwrap();
    }
    assert_ne!(early_positions, grown.positions());
    assert_eq!(a.positions(), b.positions());
    assert_eq!(grown.indices().len(), 12 * 18 * 3);
    assert!(grown.positions().iter().all(|v| v.is_finite()));
    assert!(a.tick(0.1).unwrap().is_none());
    // Facing zero places the formation ahead along negative Z.
    let center_z = grown.positions().chunks_exact(3).map(|v| v[2]).sum::<f32>() / (grown.positions().len() / 3) as f32;
    assert!(center_z < 6.0);
    for i in 0..10 { a.cast([i as f32 * 10.0, 8.0], 0.0).unwrap(); }
    let mesh = a.tick(0.1).unwrap().unwrap();
    assert_eq!(mesh.indices().len(), 4 * 12 * 18 * 3);
    assert!(mesh.positions().chunks_exact(3).all(|v| v[0] > 50.0));
    a.clear().unwrap();
    assert!(a.positions().is_empty());
    let cleared = a.tick(0.0).unwrap().unwrap();
    assert!(cleared.positions().is_empty());
}

#[test]
fn invalid_host_inputs_are_rejected() {
    let mut spell = Spells::new().unwrap();
    assert!(spell.cast([f32::NAN, 0.0], 0.0).is_err());
    assert!(spell.cast([0.0, 0.0], f32::INFINITY).is_err());
    assert!(spell.tick(f32::NAN).is_err());
    assert!(spell.tick(-0.1).is_err());
}

#[test]
fn aim_rotates_the_formation_and_growth_has_vertical_extent() {
    let mut spell = Spells::new().unwrap();
    spell.cast([0.0, 8.0], std::f32::consts::FRAC_PI_2).unwrap();
    for _ in 0..20 { spell.tick(0.1).unwrap(); }
    let points = spell.positions();
    assert!(points.chunks_exact(3).all(|p| p[0] > 2.0 && p[0] < 6.0));
    // Each 54-vertex prism has distinct bottom and top heights.
    for prism in points.chunks_exact(162) {
        let bottom = prism[1];
        let top = prism[7];
        assert!(top > bottom + 1.0);
    }
}
