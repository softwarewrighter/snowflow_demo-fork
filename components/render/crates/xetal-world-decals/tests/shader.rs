#[test]
fn footprint_shader_validates_with_the_shared_camera() {
    let source = format!(
        "{}{}",
        xetal_world_camera::SHADER,
        xetal_world_decals::SHADER
    );
    let module = naga::front::wgsl::parse_str(&source).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    assert_eq!(module.entry_points.len(), 2);
}
