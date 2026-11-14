#[test]
fn depth_only_shader_validates_with_the_shared_projection() {
    let source = concat!(
        include_str!("../src/light.wgsl"),
        include_str!("../src/depth.wgsl")
    );
    let module = naga::front::wgsl::parse_str(source).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    assert_eq!(module.entry_points.len(), 1);
    assert_eq!(module.entry_points[0].stage, naga::ShaderStage::Vertex);
}
