#[test]
fn snow_shader_parses_and_validates_without_a_gpu() {
    let source = concat!(
        include_str!("../../xetal-world-shadow/src/light.wgsl"),
        include_str!("../src/snow.wgsl")
    );
    let module = naga::front::wgsl::parse_str(source).expect("WGSL parses");
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .expect("WGSL validates against the baseline feature set");
    assert_eq!(module.entry_points.len(), 2);
}
