const BUFFERS: [wgpu::VertexBufferLayout<'static>; 1] = [wgpu::VertexBufferLayout {
    array_stride: 12,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
}];
pub(crate) fn create(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    camera: &wgpu::BindGroupLayout,
    material: u8,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("procedural ice"),
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
        label: Some("procedural ice"),
        layout: Some(&layout),
        vertex: vertex(&shader),
        fragment: Some(fragment(
            &shader,
            material,
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
        entry_point: Some("ice_vertex"),
        compilation_options: Default::default(),
        buffers: &BUFFERS,
    }
}
fn fragment<'a>(
    shader: &'a wgpu::ShaderModule,
    material: u8,
    targets: &'a [Option<wgpu::ColorTargetState>],
) -> wgpu::FragmentState<'a> {
    wgpu::FragmentState {
        module: shader,
        entry_point: Some(match material {
            1 => "slush_fragment",
            2 => "water_fragment",
            _ => "ice_fragment",
        }),
        compilation_options: Default::default(),
        targets,
    }
}
fn depth() -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth24Plus,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::LessEqual,
        stencil: Default::default(),
        bias: wgpu::DepthBiasState {
            constant: 0,
            slope_scale: 0.0,
            clamp: 0.0,
        },
    }
}
