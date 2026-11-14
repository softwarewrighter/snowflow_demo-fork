use crate::{pipeline, resources};

pub struct Shadow {
    pub view: wgpu::TextureView,
    pub group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub dirty: bool,
    pipeline: wgpu::RenderPipeline,
}

impl Shadow {
    pub fn new(device: &wgpu::Device) -> Self {
        let (view, group, layout) = resources::create(device);
        Self {
            view,
            group,
            layout,
            dirty: true,
            pipeline: pipeline::create(device),
        }
    }

    pub fn update(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        vertices: &wgpu::Buffer,
        indices: &wgpu::Buffer,
        count: u32,
    ) {
        if !self.dirty {
            return;
        }
        let depth = wgpu::RenderPassDepthStencilAttachment {
            view: &self.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("cached sun shadow"),
            depth_stencil_attachment: Some(depth),
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..count, 0, 0..1);
        self.dirty = false;
    }
}
