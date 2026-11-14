use xetal_world_model::{Player, Pose};

fn pose(x: f32, grounded: bool, impact: f32) -> Pose {
    Pose {
        point: [x, 8.0],
        yaw: 0.0,
        grounded,
        impact,
    }
}

#[test]
fn xetal_controls_footfalls_silence_in_air_landing_and_wind_fill() {
    let mut player = Player::new().unwrap();
    let first = player.update(pose(1.0, true, 0.0), 0.1).unwrap().unwrap();
    assert_eq!(first.steps, 1);
    assert_eq!(first.prints.len(), 1);
    assert_eq!(first.sound[0], 1.0);
    let airborne = player.update(pose(1.5, false, 0.0), 0.1).unwrap().unwrap();
    assert_eq!(airborne.steps, 1);
    assert_eq!(airborne.sound[0], 0.0);
    let landed = player.update(pose(1.5, true, 5.0), 0.1).unwrap().unwrap();
    assert_eq!(landed.landings, 1);
    assert_eq!(landed.prints.len(), 3);
    assert_eq!(landed.sound[0], 2.0);
    assert!(
        landed.prints[0][5] < landed.prints[1][5],
        "oldest fades first"
    );
    let mut faded = landed;
    for _ in 0..700 {
        faded = player.update(pose(1.5, true, 0.0), 0.1).unwrap().unwrap();
    }
    assert!(faded.prints.iter().all(|row| row[5] < 0.001));
    assert_eq!(faded.sound[0], 0.0);
    assert_eq!(faded.steps, 1);
}

#[test]
fn rendered_import_calls_link_to_the_real_module_reference() {
    let html = xetal_world_model::highlighted_source();
    for anchor in ["snow-terrain", "footsteps", "snow-audio", "snow-host"] {
        assert!(html.contains(&format!("library-reference.html#{anchor}")));
    }
}

#[test]
fn xetal_history_stays_bounded_after_a_long_walk() {
    let mut player = Player::new().unwrap();
    let mut commands = None;
    for i in 1..=300 {
        commands = player.update(pose(i as f32, true, 0.0), 0.1).unwrap();
        assert!(commands.as_ref().unwrap().prints.len() <= 256);
    }
    let commands = commands.unwrap();
    assert_eq!(commands.prints.len(), 256);
    assert_eq!(commands.prints[0][0], 45.0);
    assert_eq!(commands.prints[255][0], 300.0);
    assert!(
        commands
            .prints
            .iter()
            .all(|row| (0.0..=1.0).contains(&row[5]))
    );
}
