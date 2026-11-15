//! Native CPU sample; excludes WebGPU, audio and browser frame pacing.
use std::time::Instant;
use xetal_snow_app::Scene;
fn main() {
    let mut scene = Scene::new().unwrap();
    for _ in 0..25 {
        scene.update(0.0, true).unwrap();
    }
    scene.input("5", true).unwrap();
    let mut phases = [Vec::new(), Vec::new(), Vec::new()];
    for tick in 0..140 {
        let start = Instant::now();
        scene.update(0.05, false).unwrap();
        let phase = if tick < 71 {
            0
        } else if tick < 105 {
            1
        } else {
            2
        };
        phases[phase].push(start.elapsed().as_secs_f64() * 1000.0);
    }
    for (name, samples) in ["ramp/hold with terrain rebuild", "fade/tail", "settled"]
        .iter()
        .zip(&mut phases)
    {
        samples.sort_by(f64::total_cmp);
        println!(
            "{name}: median {:.3}ms, p95 {:.3}ms, worst {:.3}ms",
            samples[samples.len() / 2],
            samples[samples.len() * 95 / 100],
            samples[samples.len() - 1]
        );
    }
}
