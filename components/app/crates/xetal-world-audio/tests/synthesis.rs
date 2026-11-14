use xetal_world_audio::{Kind, samples};

#[test]
fn sound_buffers_are_deterministic_finite_quiet_and_end_smoothly() {
    for kind in [Kind::Step, Kind::Land] {
        let data = samples(kind);
        assert_eq!(data, samples(kind));
        assert!(data.len() < 10_000);
        assert!(data.iter().all(|v| v.is_finite() && v.abs() <= 1.0));
        assert_eq!(data[0], 0.0);
        assert!(data.last().unwrap().abs() < 0.0001);
        assert!(data.iter().any(|v| v.abs() > 0.01));
    }
    assert_ne!(samples(Kind::Step), samples(Kind::Land));
}
