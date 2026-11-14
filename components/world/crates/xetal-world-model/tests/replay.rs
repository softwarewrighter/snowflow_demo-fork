use xetal_world_model::{Demo, Progress};

const BEFORE: &str = include_str!("fixtures/snowfield-mutable.xtl");

fn settle(demo: &mut Demo) -> Progress {
    for _ in 0..500 {
        let progress = demo.step(997).unwrap();
        if progress != Progress::Running {
            return progress;
        }
    }
    panic!("replay did not settle");
}

#[test]
fn immutable_state_replays_the_legacy_geometry_exactly() {
    let mut before = Demo::from_source(BEFORE).unwrap();
    let mut after = Demo::new().unwrap();
    assert_eq!(settle(&mut before), settle(&mut after));
    assert_eq!(before.mesh().unwrap(), after.mesh().unwrap());
    for event in [
        "tick 0.08",
        "tick 0.25",
        "down 0 0",
        "tick 0.08",
        "choose 1 0",
        "down 0 0",
        "key Home",
        "tick 1.25",
        "key Home",
        "end",
    ] {
        before.event(event).unwrap();
        after.event(event).unwrap();
        assert_eq!(settle(&mut before), settle(&mut after), "{event}");
        assert_eq!(before.mesh().unwrap(), after.mesh().unwrap(), "{event}");
    }
}
