use xetal_world_navigation::Motion;

#[test]
fn jumping_returns_to_ground_and_cannot_restart_in_midair() {
    let start = Motion {
        height: 1.7,
        velocity: 0.0,
        grounded: true,
    };
    let airborne = start.step(1.0 / 120.0, 1.7, true);
    assert!(airborne.height > start.height);
    assert_eq!(
        airborne.step(0.01, 1.7, true),
        airborne.step(0.01, 1.7, false)
    );
    let mut state = airborne;
    for _ in 0..240 {
        state = state.step(1.0 / 120.0, 1.7, false);
    }
    assert_eq!(state, start);
}

#[test]
fn falling_is_consistent_across_step_sizes_and_stops_at_raised_terrain() {
    let start = Motion {
        height: 20.0,
        velocity: 0.0,
        grounded: false,
    };
    let mut fine = start;
    let mut coarse = start;
    for _ in 0..120 {
        fine = fine.step(1.0 / 120.0, 1.7, false);
    }
    for _ in 0..60 {
        coarse = coarse.step(1.0 / 60.0, 1.7, false);
    }
    assert!((fine.height - coarse.height).abs() < 0.001);
    assert_eq!(fine.step(0.01, 30.0, false).height, 30.0);
}

#[test]
fn jump_input_survives_a_frame_shorter_than_one_physics_step() {
    let mut walker = xetal_world_navigation::Walker::default();
    let terrain = [-20.0, 0.0, -20.0, 20.0, 0.0, -20.0, 0.0, 0.0, 20.0];
    walker.ground(&terrain);
    walker.simulate(0.001, true, &terrain);
    walker.simulate(0.008, false, &terrain);
    assert!(walker.position[1] > 1.7);
}

#[test]
fn ordinary_downhill_and_uphill_walking_keep_ground_contact() {
    let slope = [-100.0,-10.0,-100.0,100.0,-10.0,-100.0,0.0,10.0,100.0];
    for direction in [-1.0,1.0] {
        let mut walker = xetal_world_navigation::Walker::default();
        walker.ground(&slope);
        for _ in 0..120 {
            walker.advance(1.0/60.0, direction, 0.0);
            walker.simulate(1.0/60.0, false, &slope);
            assert!(walker.motion.grounded, "walking contact was dropped on a slope");
        }
    }
}
