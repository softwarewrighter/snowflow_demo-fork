use xetal_world_metrics::Meter;
#[test]
fn sixty_hz_is_measured_from_intervals_and_background_gaps_reset() {
    let (mut meter, _) = Meter::default().record(0.0);
    let mut result = None;
    for i in 1..=31 {
        (meter, result) = meter.record(f64::from(i) * 1000.0 / 60.0);
        if result.is_some() {
            break;
        }
    }
    let result = result.unwrap();
    assert!((result.fps - 60.0).abs() < 0.01);
    assert!((result.mean_ms - 1000.0 / 60.0).abs() < 0.01);
    assert!(meter.record(20_000.0).1.is_none());
}
