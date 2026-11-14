#[test]
fn ice_shader_validates_with_shared_camera() {
    let source = format!(
        "{}{}",
        xetal_world_camera::SHADER,
        xetal_world_objects::SHADER
    );
    let module = naga::front::wgsl::parse_str(&source).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
}
#[test]
fn object_upload_rejects_invalid_payloads() {
    use xetal_world_objects::validate;
    assert!(validate(&[]).is_ok());
    assert!(validate(&[0.0; 9]).is_ok());
    assert!(validate(&[0.0; 8]).is_err());
    assert!(validate(&[f32::NAN; 9]).is_err());
    assert!(validate(&vec![0.0; xetal_world_objects::MAX_VERTICES * 3 + 9]).is_err());
}
