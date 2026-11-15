//! Native Scene CPU sample; excludes GPU/audio/browser pacing and footprint work.
use std::time::Instant;
use xetal_snow_app::Scene;
fn main() {
    let mut scene = Scene::new().unwrap();
    for _ in 0..1000 {
        scene.update(0.0, true).unwrap();
        if scene.world.resident() == 25 {
            break;
        }
    }
    scene.input("2", true).unwrap();
    for phase in ["held", "released"] {
        if phase == "released" {
            scene.input("2", false).unwrap();
        }
        let mut samples = Vec::new();
        for i in 0..80 {
            if phase == "held" {
                scene
                    .input(
                        if i % 12 < 6 {
                            "ArrowLeft"
                        } else {
                            "ArrowRight"
                        },
                        true,
                    )
                    .unwrap();
            }
            let start = Instant::now();
            scene.update(0.05, false).unwrap();
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        samples.sort_by(f64::total_cmp);
        println!(
            "{phase}: median {:.3}ms, p95 {:.3}ms, worst {:.3}ms; cumulative terrain edits {}",
            samples[40], samples[76], samples[79], scene.world.edit_revision
        );
    }
}
