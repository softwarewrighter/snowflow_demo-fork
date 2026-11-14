use xetal_world_ribbon::{Pose, Ribbon};
fn pose(yaw: f32) -> Pose {
    Pose {
        origin: [0.0, 2.0, 8.0],
        aim: [yaw, -0.12],
    }
}
#[test]
fn held_ribbon_records_history_then_throws_and_retires() {
    let mut ribbon = Ribbon::new().unwrap();
    assert!(
        ribbon
            .update(pose(0.0), 0.05, |_| None)
            .unwrap()
            .mesh
            .is_none()
    );
    ribbon.press(pose(0.0)).unwrap();
    for _ in 0..20 {
        ribbon.update(pose(0.0), 0.05, |_| None).unwrap();
    }
    let previous = ribbon.positions().to_vec();
    assert!(!previous.is_empty());
    for _ in 0..10 {
        ribbon.update(pose(0.6), 0.05, |_| None).unwrap();
    }
    assert_ne!(previous, ribbon.positions());
    let before_release = ribbon.tip();
    ribbon.release(pose(0.6)).unwrap();
    for _ in 0..5 {
        ribbon.update(pose(-1.0), 0.05, |_| None).unwrap();
    }
    assert!((ribbon.tip()[0] - before_release[0]).hypot(ribbon.tip()[2] - before_release[2]) > 0.5);
    assert_eq!(ribbon.mode(), 2);
    assert!(ribbon.positions().len() <= 496 * 9);
    for _ in 0..65 {
        ribbon.update(pose(0.0), 0.05, |_| None).unwrap();
    }
    assert!(ribbon.positions().is_empty());
    assert_eq!(ribbon.mode(), 0);
}
#[test]
fn contact_scores_once_per_new_point_and_reset_clears_geometry() {
    let mut ribbon = Ribbon::new().unwrap();
    ribbon.press(pose(0.0)).unwrap();
    let mut scored = false;
    for _ in 0..30 {
        let update = ribbon.update(pose(0.0), 0.05, |_| Some(10.0)).unwrap();
        scored |= update.score.is_some();
        assert!(ribbon.positions().iter().all(|v| v.is_finite()));
    }
    assert!(scored);
    ribbon.clear().unwrap();
    assert!(
        ribbon
            .update(pose(0.0), 0.0, |_| None)
            .unwrap()
            .mesh
            .unwrap()
            .positions()
            .is_empty()
    );
    assert!(
        ribbon
            .press(Pose {
                origin: [f32::NAN; 3],
                aim: [0.0; 2]
            })
            .is_err()
    );
    assert!(ribbon.update(pose(0.0), -0.1, |_| None).is_err());
}

#[test]
fn recast_during_flight_uses_fresh_pose_and_matches_replay() {
    let mut a = Ribbon::new().unwrap();
    let mut b = Ribbon::new().unwrap();
    for ribbon in [&mut a, &mut b] {
        ribbon.press(pose(0.0)).unwrap();
        for _ in 0..10 {
            ribbon.update(pose(0.0), 0.05, |_| None).unwrap();
        }
        ribbon.release(pose(0.5)).unwrap();
        for _ in 0..5 {
            ribbon.update(pose(-1.0), 0.05, |_| None).unwrap();
        }
        ribbon
            .press(Pose {
                origin: [20.0, 2.0, 8.0],
                aim: [-0.5, -0.2],
            })
            .unwrap();
        ribbon
            .update(
                Pose {
                    origin: [20.0, 2.0, 8.0],
                    aim: [-0.5, -0.2],
                },
                0.05,
                |_| None,
            )
            .unwrap();
    }
    assert!(a.tip()[0] > 15.0);
    assert_eq!(a.positions(), b.positions());
}
