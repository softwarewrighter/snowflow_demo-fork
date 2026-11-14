use xetal_world_model::{Demo, Progress};

fn settle(demo: &mut Demo) {
    for _ in 0..100 {
        if demo.step(20_000).unwrap() == Progress::Waiting {
            return;
        }
    }
    panic!("did not reach event wait");
}

#[test]
fn terrain_is_generated_by_the_persistent_program_and_reset_by_an_event() {
    let mut demo = Demo::new().unwrap();
    settle(&mut demo);
    let original = demo.mesh().unwrap();
    assert_eq!(original.indices().len(), 128 * 3);
    let p = original.positions();
    let normal_y = (p[5] - p[2]) * (p[6] - p[0]) - (p[3] - p[0]) * (p[8] - p[2]);
    assert!(normal_y > 0.0, "terrain triangles must face upward");
    demo.event("tick 0.25").unwrap();
    settle(&mut demo);
    assert_ne!(demo.mesh().unwrap().positions(), original.positions());
    demo.event("key Home").unwrap();
    settle(&mut demo);
    assert_eq!(demo.mesh().unwrap(), original);
}

#[test]
fn input_changes_the_program_geometry_and_bad_source_is_reported() {
    let mut demo = Demo::new().unwrap();
    settle(&mut demo);
    let original = demo.mesh().unwrap();
    demo.event("down 0 0").unwrap();
    settle(&mut demo);
    assert_ne!(demo.mesh().unwrap().positions(), original.positions());
    assert!(Demo::from_source("1 + \"wrong\"").is_err());
}
