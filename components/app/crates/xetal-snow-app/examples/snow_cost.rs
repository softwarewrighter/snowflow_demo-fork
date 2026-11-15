//! Repeatable native fine-layer CPU and resource sample; GPU/audio excluded.
use std::time::Instant;
use xetal_snow_app::Scene;
fn main() {
    let mut scene = Scene::new().unwrap();
    for _ in 0..25 {
        scene.update(0.0, true).unwrap();
    }
    for phase in ["walking", "resting"] {
        scene.input("w", phase == "walking").unwrap();
        let mut samples = Vec::new();
        for _ in 0..180 {
            let start = Instant::now();
            scene.update(0.05, false).unwrap();
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        samples.sort_by(f64::total_cmp);
        println!(
            "{phase}: median {:.3}ms p95 {:.3}ms worst {:.3}ms; {} tiles, {} payload bytes, {} triangles",
            samples[90],
            samples[171],
            samples[179],
            scene.detail.resident(),
            scene.detail.memory_bytes(),
            scene.terrain_triangles
        );
    }
}
