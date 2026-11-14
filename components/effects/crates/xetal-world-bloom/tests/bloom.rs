use xetal_world_bloom::Bloom;
#[test]
fn bloom_bursts_once_then_leaves_four_seconds_of_fallout() {
    let mut bloom = Bloom::new().unwrap();
    assert_eq!(bloom.targeting(), [24.0, 6.0]);
    assert!(bloom.tick(0.1).unwrap().crater.is_none());
    bloom.cast([3.0, 2.0, -4.0]).unwrap();
    let first = bloom.tick(0.1).unwrap();
    assert_eq!(first.crater.unwrap(), [1.0, 3.0, -4.0]);
    let mut visible = false;
    for _ in 0..9 {
        let update = bloom.tick(0.1).unwrap();
        assert!(
            update.crater.is_none(),
            "one cast creates exactly one crater"
        );
        if let Some(mesh) = update.mesh {
            assert!(mesh.indices().len() <= 480 * 3);
        }
        if let Some(particles) = update.particles {
            assert!(particles.len() <= 512 * 6);
            visible |= particles.as_chunks::<6>().0.iter().any(|p| p[4] > 0.0);
        }
    }
    assert!(visible);
    for _ in 0..10 {
        bloom.tick(0.1).unwrap();
    }
    assert!(
        bloom.positions().is_empty(),
        "column collapses before fallout ends"
    );
    assert!(
        bloom
            .particles()
            .as_chunks::<6>()
            .0
            .iter()
            .any(|p| p[4] > 0.0)
    );
    for _ in 0..40 {
        bloom.tick(0.1).unwrap();
    }
    assert!(bloom.particles().is_empty());
    bloom.cast([3.0, 2.0, -4.0]).unwrap();
    assert_eq!(bloom.tick(0.1).unwrap().crater.unwrap()[0], 2.0);
    bloom.clear().unwrap();
    let cleared = bloom.tick(0.0).unwrap();
    assert!(cleared.mesh.unwrap().positions().is_empty());
    assert!(cleared.particles.unwrap().is_empty());
}
#[test]
fn invalid_native_inputs_are_rejected() {
    let mut bloom = Bloom::new().unwrap();
    assert!(bloom.cast([f32::NAN; 3]).is_err());
    assert!(bloom.tick(-0.1).is_err());
    assert!(bloom.tick(f32::INFINITY).is_err());
}

#[test]
fn particle_motion_is_deterministic_finite_and_descends_during_fallout() {
    let mut a = Bloom::new().unwrap();
    let mut b = Bloom::new().unwrap();
    for bloom in [&mut a, &mut b] {
        bloom.cast([0.0, 2.0, 0.0]).unwrap();
    }
    for _ in 0..20 {
        a.tick(0.1).unwrap();
        b.tick(0.1).unwrap();
    }
    assert_eq!(a.particles(), b.particles());
    let before = a.particles().to_vec();
    a.tick(0.1).unwrap();
    assert!(a.particles().iter().all(|v| v.is_finite()));
    assert!(
        before
            .as_chunks::<6>()
            .0
            .iter()
            .zip(a.particles().as_chunks::<6>().0)
            .any(|(p, q)| p[4] > 0.0 && q[4] > 0.0 && q[1] < p[1])
    );
}
