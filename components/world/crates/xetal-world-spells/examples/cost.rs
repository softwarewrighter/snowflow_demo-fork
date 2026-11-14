use std::time::Instant;
use xetal_world_spells::Spells;
fn main() {
    let mut spells = Spells::new().unwrap();
    for i in 0..4 {
        spells.cast([i as f32 * 3.0, 8.0], 0.0).unwrap();
    }
    let start = Instant::now();
    for _ in 0..18 {
        spells.tick(0.1).unwrap();
    }
    let growth = start.elapsed().as_secs_f64() * 1000.0 / 18.0;
    let start = Instant::now();
    for _ in 0..100 {
        spells.tick(0.1).unwrap();
    }
    println!(
        "growth mean ms/tick: {growth:.3}; settled mean ms/tick: {:.3}; vertices: {}",
        start.elapsed().as_secs_f64() * 10.0,
        spells.positions().len() / 3
    );
}
