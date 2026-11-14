use xetal_world_model::{Demo, Progress};
fn settle(demo: &mut Demo) {
    for _ in 0..100 {
        if demo.step(8_000).unwrap() == Progress::Waiting {
            return;
        }
    }
    panic!("test program did not wait");
}
#[test]
fn numeric_publication_is_typed_and_missing_fields_are_rejected() {
    let mut demo = Demo::from_source("frame! := (0 3 3 r_eshape 0.0, 1.0 2.0 3.0)\nu:w_atch := { @ -> frame! }\n'u:w_atch\n[]E_VENT @").unwrap();
    settle(&mut demo);
    assert_eq!(demo.array(1).unwrap(), (vec![3], vec![1.0, 2.0, 3.0]));
    assert!(demo.array(2).is_err());
    let mut invalid = Demo::from_source(
        "frame! := (0 3 3 r_eshape 0.0, 1.0)\nu:w_atch := { @ -> frame! }\n'u:w_atch\n[]E_VENT @",
    )
    .unwrap();
    settle(&mut invalid);
    assert!(invalid.array(1).is_err());
}
