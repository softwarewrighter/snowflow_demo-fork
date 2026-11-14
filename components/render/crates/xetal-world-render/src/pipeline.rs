const BUFFERS: [wgpu::VertexBufferLayout<'static>; 1] = [wgpu::VertexBufferLayout {
    array_stride: 12,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
}];

pub(crate) fn create(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    group: &wgpu::BindGroupLayout,
    shadow: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = shader(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[group, shadow],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("snow mesh"),
        layout: Some(&layout),
        vertex: vertex(&shader),
        fragment: Some(fragment(&shader, &[Some(format.into())])),
        primitive: Default::default(),
        depth_stencil: Some(depth_state()),
        multisample: Default::default(),
        multiview: None,
        cache: None,
    })
}

fn shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("procedural snow"),
        source: wgpu::ShaderSource::Wgsl(
            format!(
                "{}{}{}",
                xetal_world_camera::SHADER,
                include_str!("../../xetal-world-shadow/src/light.wgsl"),
                include_str!("snow.wgsl")
            )
            .into(),
        ),
    })
}

fn vertex(shader: &wgpu::ShaderModule) -> wgpu::VertexState<'_> {
    wgpu::VertexState {
        module: shader,
        entry_point: Some("vertex"),
        compilation_options: Default::default(),
        buffers: &BUFFERS,
    }
}

fn fragment<'a>(
    shader: &'a wgpu::ShaderModule,
    targets: &'a [Option<wgpu::ColorTargetState>],
) -> wgpu::FragmentState<'a> {
    wgpu::FragmentState {
        module: shader,
        entry_point: Some("fragment"),
        compilation_options: Default::default(),
        targets,
    }
}

fn depth_state() -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth24Plus,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: Default::default(),
        bias: Default::default(),
    }
}
