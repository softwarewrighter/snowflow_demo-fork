//! Native CPU costs, including mesh merging; not a browser FPS measurement.
use std::time::Instant;
use xetal_world_stream::World;

fn fill(world: &mut World, position: [f32; 3]) -> usize {
    let mut bytes = 0;
    for _ in 0..1000 {
        if let Some(mesh) = world.step(position, 2_000).unwrap() { bytes = mesh.byte_len(); }
        if world.resident() == 25 { return bytes; }
    }
    panic!("ring did not fill");
}

fn run() -> (f64,f64,f64,usize) {
    let start = Instant::now();
    let mut world = World::new().unwrap();
    let bytes = fill(&mut world, [0.0,0.0,8.0]);
    let initial = start.elapsed().as_secs_f64()*1000.0;
    let start = Instant::now();
    fill(&mut world, [40.0,0.0,8.0]);
    let shift = start.elapsed().as_secs_f64()*1000.0;
    let generated = world.generated;
    let start = Instant::now();
    for _ in 0..10000 { assert!(std::hint::black_box(&mut world).step(std::hint::black_box([40.0,0.0,8.0]), 2_000).unwrap().is_none()); }
    let idle = start.elapsed().as_secs_f64()*1000000.0/10000.0;
    assert_eq!(world.generated, generated);
    (initial,shift,idle,bytes)
}

fn main() {
    let samples: Vec<_> = (0..7).map(|_| run()).collect();
    println!("{{\"initial_25_chunks_ms\":{:?},\"shift_5_chunks_ms\":{:?},\"idle_us_per_update\":{:?},\"merged_mesh_bytes\":{}}}", samples.iter().map(|s|s.0).collect::<Vec<_>>(),samples.iter().map(|s|s.1).collect::<Vec<_>>(),samples.iter().map(|s|s.2).collect::<Vec<_>>(),samples[0].3);
}
