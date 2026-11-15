use std::rc::Rc;
use xetal_app_protocol::{Cache, decode};
use xetal_app_session::SOURCE;
use xetal_value::Value;
use xetal_world_model::{Demo, Progress};
fn publication() -> Value<'static> {
    let mut demo = Demo::from_source(SOURCE).unwrap();
    while demo.step(8000).unwrap() != Progress::Waiting {}
    demo.publication().unwrap()
}
fn replace(value: &Value<'static>, index: usize, next: Value<'static>) -> Value<'static> {
    let Value::Tuple(fields) = value else {
        panic!("expected tuple")
    };
    let mut fields = fields.to_vec();
    fields[index] = next;
    Value::Tuple(fields.into())
}
#[test]
fn malformed_publications_are_rejected_before_native_capabilities() {
    let valid = publication();
    let Value::Tuple(f) = &valid else { panic!() };
    let bad_audio = if let Value::Tuple(a) = &f[4] {
        if let Value::Array(command) = &a[1] {
            replace(
                &f[4],
                1,
                Value::Array(Rc::new(
                    command.map(|_| Ok::<_, ()>(Value::Float(999.0))).unwrap(),
                )),
            )
        } else {
            panic!()
        }
    } else {
        panic!()
    };
    let cases = [
        replace(&valid, 11, Value::Int(99)),
        replace(&valid, 8, replace(&f[8], 4, Value::Float(f64::NAN))),
        replace(&valid, 8, replace(&f[8], 3, Value::Int(-1))),
        replace(&valid, 4, bad_audio),
        replace(&valid, 3, Value::Float(0.0)),
        replace(&valid, 6, f[3].clone()),
    ];
    for bad in cases {
        assert!(decode(&bad, &mut Cache::default()).is_err());
    }
    assert!(decode(&valid, &mut Cache::default()).is_ok());
}
#[test]
fn duplicate_resources_and_oversized_geometry_are_rejected() {
    let valid = publication();
    let Value::Tuple(f) = &valid else { panic!() };
    let Value::Tuple(resources) = &f[1] else {
        panic!()
    };
    let duplicate = replace(
        &valid,
        1,
        Value::Tuple(vec![resources[0].clone(), resources[0].clone()].into()),
    );
    assert!(decode(&duplicate, &mut Cache::default()).is_err());
    let mut large = Demo::from_source("frame! := 865 3 3 r_eshape 0.0; u:w_atch := { @ -> frame! }; 'u:w_atch; event := []E_VENT @").unwrap();
    while large.step(8000).unwrap() != Progress::Waiting {}
    let resource = replace(&resources[0], 3, large.publication().unwrap());
    let invalid = replace(&valid, 1, Value::Tuple(vec![resource].into()));
    assert!(decode(&invalid, &mut Cache::default()).is_err());
}
