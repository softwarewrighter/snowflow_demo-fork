use xetal_world_model::{Demo, Progress};

fn settle(demo: &mut Demo) {
    for _ in 0..100 {
        if demo.step(20_000).unwrap() != Progress::Running {
            return;
        }
    }
    panic!("program did not settle");
}

#[test]
fn slices_are_bounded_and_events_require_an_event_wait() {
    let mut demo = Demo::new().unwrap();
    assert!(demo.event("tick 0.1").is_err());
    assert_eq!(demo.step(1).unwrap(), Progress::Running);
    settle(&mut demo);
    demo.event("end").unwrap();
    settle(&mut demo);
    assert_eq!(demo.step(1).unwrap(), Progress::Done);
}

#[test]
fn separate_programs_do_not_share_published_meshes() {
    let mut first = Demo::new().unwrap();
    let mut second = Demo::new().unwrap();
    settle(&mut first);
    settle(&mut second);
    let original = second.mesh().unwrap();
    first.event("choose 1 0").unwrap();
    settle(&mut first);
    assert_ne!(first.mesh().unwrap(), original);
    assert_eq!(second.mesh().unwrap(), original);
}

#[test]
fn typed_snapshot_rejects_integer_coordinates_that_lose_float_precision() {
    let source =
        "mesh! := 1 3 3 r_eshape 9007199254740993\nu:w_atch := { @ -> mesh! }\n'u:w_atch\n";
    let mut demo = Demo::from_source(source).unwrap();
    settle(&mut demo);
    assert!(demo.mesh().is_err());
}
