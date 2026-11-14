//! Native Scene CPU sample; excludes GPU/audio/browser pacing and footprint work.
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
    scene.ribbon_input(true).unwrap();
    for phase in ["held", "released"] {
        if phase == "released" {
            scene.ribbon_input(false).unwrap();
        }
        let mut samples = Vec::new();
        for i in 0..80 {
            if phase == "held" {
                scene.walker.yaw = (i as f32 * 0.08).sin() * 0.35;
            }
            let start = Instant::now();
            scene.update(0.05, [false; 4], false, false).unwrap();
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        samples.sort_by(f64::total_cmp);
        println!(
            "{phase}: median {:.3}ms, p95 {:.3}ms, worst {:.3}ms; cumulative terrain edits {}",
            samples[40], samples[76], samples[79], scene.world.edit_revision
        );
    }
}
