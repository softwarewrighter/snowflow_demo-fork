use xetal_app_session::{Pose, Session};
#[test]
fn boot_samples_have_depth_edges_and_matching_cross_tile_borders() {
    let mut app = Session::new().unwrap();
    app.pose(Pose {
        position: [1.95, 4.0, 8.0],
        aim: [0.0, 0.0],
        grounded: true,
        impact: 0.0,
    })
    .unwrap();
    app.pose(Pose {
        position: [1.95, 4.0, 7.0],
        aim: [0.0, 0.0],
        grounded: true,
        impact: 0.0,
    })
    .unwrap();
    let a = app.detail_tile((0, 3), 0.1).unwrap();
    let b = app.detail_tile((1, 3), 0.1).unwrap();
    assert!(a.as_chunks::<3>().0.iter().any(|p| p[1] < -0.01));
    assert!(a.as_chunks::<3>().0.iter().any(|p| p[1] > 0.001));
    for z in 0..=20 {
        assert_eq!(a[(20 * 21 + z) * 3 + 1], b[z * 3 + 1]);
    }
    assert_eq!(a, app.detail_tile((0, 3), 0.1).unwrap());
    app.input("c", true).unwrap();
    assert!(
        app.detail_tile((0, 3), 0.1)
            .unwrap()
            .as_chunks::<3>()
            .0
            .iter()
            .all(|p| p[1] == 0.0)
    );
}
#[test]
fn finer_negative_coordinate_stamps_replay_across_borders_and_reject_bad_spacing() {
    let mut app = Session::new().unwrap();
    assert!(app.detail_tile((0, 0), 0.0).is_err());
    assert!(app.detail_tile((0, 0), 0.07).is_err());
    app.pose(Pose {
        position: [-0.05, 4.0, -0.1],
        aim: [0.4, 0.0],
        grounded: true,
        impact: 0.0,
    })
    .unwrap();
    app.pose(Pose {
        position: [-0.05, 4.0, -1.1],
        aim: [0.4, 0.0],
        grounded: true,
        impact: 0.0,
    })
    .unwrap();
    let left = app.detail_tile((-1, -1), 0.0625).unwrap();
    let right = app.detail_tile((0, -1), 0.0625).unwrap();
    assert_eq!(left.len(), 33 * 33 * 3);
    for z in 0..33 {
        assert_eq!(left[(32 * 33 + z) * 3 + 1], right[z * 3 + 1]);
    }
    assert!(
        left.as_chunks::<3>()
            .0
            .iter()
            .chain(right.as_chunks::<3>().0)
            .all(|p| p[1] >= -0.075 && p[1] <= 0.025)
    );
    assert_eq!(left, app.detail_tile((-1, -1), 0.0625).unwrap());
}
