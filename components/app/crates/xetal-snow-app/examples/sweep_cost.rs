//! Native Scene CPU timing; excludes browser rendering, audio and frame pacing.
use std::time::Instant;
use xetal_snow_app::Scene;
fn main() {
    let mut scene = Scene::new().unwrap();
    for _ in 0..1000 {
        scene.update(0.0, [false; 4], false, true).unwrap();
        if scene.world.resident() == 25 {
            break;
        }
    }
    for cast in 1..=8 {
        scene.sweep.cast([0.0, 8.0], 0.0).unwrap();
        let mut samples = Vec::new();
        for _ in 0..24 {
            let start = Instant::now();
            scene.update(0.1, [false; 4], false, false).unwrap();
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        samples.sort_by(f64::total_cmp);
        println!(
            "cast {cast}: median {:.3}ms, p95 {:.3}ms, worst {:.3}ms",
            samples[12], samples[22], samples[23]
        );
    }
}
