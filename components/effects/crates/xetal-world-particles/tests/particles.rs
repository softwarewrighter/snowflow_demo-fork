#[test]
fn particle_shader_validates_with_the_shared_camera() {
    let source = format!(
        "{}{}",
        xetal_world_camera::SHADER,
        xetal_world_particles::SHADER
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
fn particle_uploads_are_bounded_and_validate_sizes_opacity_and_numbers() {
    use xetal_world_particles::validate;
    assert!(validate(&[]).is_ok());
    assert!(validate(&[0.0, 0.0, 0.0, 0.1, 1.0, 0.0]).is_ok());
    assert!(validate(&[0.0; 5]).is_err());
    assert!(validate(&[0.0; 6]).is_err());
    assert!(validate(&[0.0, 0.0, 0.0, 0.1, 1.1, 0.0]).is_err());
    assert!(validate(&[f32::NAN; 6]).is_err());
    assert!(validate(&vec![0.0; (xetal_world_particles::CAPACITY + 1) * 6]).is_err());
}
