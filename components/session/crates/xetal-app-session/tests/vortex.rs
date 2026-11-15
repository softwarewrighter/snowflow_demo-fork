use xetal_app_session::Session;
#[test]
fn vortex_is_configured_in_xetal_and_lifts_then_restores_surface() {
    let mut app = Session::new().unwrap();
    assert_eq!(app.read().unwrap().buttons[4].key, "5");
    let original = app.chunk((0, 0)).unwrap().positions().to_vec();
    app.input("5", true).unwrap();
    for _ in 0..10 {
        app.tick(0.1).unwrap();
    }
    let frame = app.read().unwrap();
    assert_eq!(frame.stats["vortex"], 1536.0);
    assert!(!frame.particles.is_empty() && frame.particles.len() <= 1024 * 6);
    assert!(app.chunk((0, 0)).unwrap().positions() != original);
    for _ in 0..50 {
        app.tick(0.1).unwrap();
    }
    assert_eq!(app.read().unwrap().stats["vortex"], 0.0);
    assert!(app.read().unwrap().particles.is_empty());
    assert_eq!(app.chunk((0, 0)).unwrap().positions(), original);
}
#[test]
fn vortex_and_existing_effects_share_bounded_resources_and_reset() {
    let mut app = Session::new().unwrap();
    for key in ["1", "4", "5"] {
        app.input(key, true).unwrap();
    }
    app.input("3", true).unwrap();
    app.respond(1, Some([0.0, 0.0, 0.0])).unwrap();
    for _ in 0..20 {
        app.tick(0.1).unwrap();
        let frame = app.read().unwrap();
        assert!(
            frame
                .resources
                .iter()
                .all(|r| r.mesh.indices().len() <= 2400 * 3)
        );
        assert!(frame.particles.len() <= 1024 * 6);
    }
    app.input("r", true).unwrap();
    let frame = app.read().unwrap();
    assert!(
        frame
            .resources
            .iter()
            .all(|r| r.mesh.positions().is_empty())
    );
    assert!(frame.particles.is_empty());
}
#[test]
fn vortex_is_deterministic_follows_pose_and_freezes_when_paused() {
    use xetal_app_session::Pose;
    let mut a = Session::new().unwrap();
    let mut b = Session::new().unwrap();
    a.input("5", true).unwrap();
    b.input("5", true).unwrap();
    for _ in 0..10 {
        a.tick(0.1).unwrap();
        b.tick(0.1).unwrap();
    }
    assert_eq!(a.read().unwrap().particles, b.read().unwrap().particles);
    a.pose(Pose {
        position: [12.0, 4.0, 8.0],
        aim: [0.0, -0.12],
        grounded: true,
        impact: 0.0,
    })
    .unwrap();
    a.tick(0.1).unwrap();
    b.tick(0.1).unwrap();
    let moved = a.read().unwrap();
    let fixed = b.read().unwrap();
    let moved = &moved
        .resources
        .iter()
        .find(|r| r.material == "slush")
        .unwrap()
        .mesh;
    let fixed = &fixed
        .resources
        .iter()
        .find(|r| r.material == "slush")
        .unwrap()
        .mesh;
    for (p, q) in moved
        .positions()
        .as_chunks::<3>()
        .0
        .iter()
        .zip(fixed.positions().as_chunks::<3>().0)
    {
        assert!((p[0] - q[0] - 12.0).abs() < 0.00001);
        assert_eq!(p[1], q[1]);
        assert_eq!(p[2], q[2]);
    }
    a.input("p", true).unwrap();
    let before = a.read().unwrap();
    a.tick(0.1).unwrap();
    let after = a.read().unwrap();
    assert_eq!(before.resources[1].revision, after.resources[1].revision);
    assert_eq!(before.particles, after.particles);
}
#[test]
fn settling_preserves_existing_carved_ground_and_recast_stays_bounded() {
    let mut app = Session::new().unwrap();
    app.input("1", true).unwrap();
    for _ in 0..30 {
        app.tick(0.1).unwrap();
    }
    let carved = app.chunk((0, 0)).unwrap().positions().to_vec();
    for _ in 0..4 {
        app.input("5", true).unwrap();
        app.tick(0.1).unwrap();
    }
    for _ in 0..60 {
        app.tick(0.1).unwrap();
    }
    assert_eq!(app.chunk((0, 0)).unwrap().positions(), carved);
}
