use std::collections::BTreeMap;
use xetal_world_model::{Demo, Progress};

fn settle(demo: &mut Demo) {
    for _ in 0..200 {
        if demo.step(8_000).unwrap() == Progress::Waiting {
            return;
        }
    }
    panic!("chunk did not settle");
}

#[test]
fn positive_and_negative_neighbors_share_identical_world_space_edges() {
    let mut demo = Demo::new().unwrap();
    settle(&mut demo);
    let original = demo.mesh().unwrap();
    let edge = |positions: &[f32], axis: usize, x: f32| -> BTreeMap<u32, u32> {
        positions
            .as_chunks::<3>()
            .0
            .iter()
            .filter(|p| p[axis] == x)
            .map(|p| (p[if axis == 0 { 2 } else { 0 }].to_bits(), p[1].to_bits()))
            .collect()
    };
    demo.event("choose 1 0").unwrap();
    settle(&mut demo);
    assert_eq!(
        edge(original.positions(), 0, 32.0),
        edge(demo.mesh().unwrap().positions(), 0, 32.0)
    );
    demo.event("choose -1 0").unwrap();
    settle(&mut demo);
    assert_eq!(
        edge(original.positions(), 0, 0.0),
        edge(demo.mesh().unwrap().positions(), 0, 0.0)
    );
    demo.event("choose 0 1").unwrap(); settle(&mut demo);
    assert_eq!(edge(original.positions(), 2, 32.0), edge(demo.mesh().unwrap().positions(), 2, 32.0));
    demo.event("choose 0 -1").unwrap(); settle(&mut demo);
    assert_eq!(edge(original.positions(), 2, 0.0), edge(demo.mesh().unwrap().positions(), 2, 0.0));
    demo.event("choose 0 0").unwrap();
    settle(&mut demo);
    assert_eq!(
        demo.mesh().unwrap(),
        original,
        "revisiting a chunk must reproduce it exactly"
    );
}
