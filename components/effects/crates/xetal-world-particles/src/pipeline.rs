const BUFFERS: [wgpu::VertexBufferLayout<'static>; 1] = [wgpu::VertexBufferLayout {
    array_stride: 24,
    step_mode: wgpu::VertexStepMode::Instance,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Float32, 3 => Float32],
}];
pub(crate) fn create(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    camera: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("snow particles"),
        source: wgpu::ShaderSource::Wgsl(
            format!("{}{}", xetal_world_camera::SHADER, crate::SHADER).into(),
        ),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[camera],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("snow particles"),
        layout: Some(&layout),
        vertex: vertex(&shader),
        fragment: Some(fragment(
            &shader,
            &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        )),
        primitive: Default::default(),
        depth_stencil: Some(depth()),
        multisample: Default::default(),
        multiview: None,
        cache: None,
    })
}

fn vertex(shader: &wgpu::ShaderModule) -> wgpu::VertexState<'_> {
    wgpu::VertexState {
        module: shader,
        entry_point: Some("particle_vertex"),
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
        entry_point: Some("particle_fragment"),
        compilation_options: Default::default(),
        targets,
    }
}
fn depth() -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth24Plus,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::LessEqual,
        stencil: Default::default(),
        bias: wgpu::DepthBiasState {
            constant: 0,
            slope_scale: 0.0,
            clamp: 0.0,
        },
    }
}
