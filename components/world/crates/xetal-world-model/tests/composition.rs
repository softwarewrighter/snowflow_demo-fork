use xetal_world_model::{Demo, Progress};
#[test]
fn application_composition_typechecks_and_waits() {
    let mut demo = Demo::from_source(include_str!("../../../demos/App.xtl")).unwrap();
    for _ in 0..1000 {
        if demo.step(8000).unwrap() == Progress::Waiting {
            assert!(demo.publication().is_ok());
            return;
        }
    }
    panic!("application did not reach event boundary");
}
