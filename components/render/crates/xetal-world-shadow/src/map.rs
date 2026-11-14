use crate::{pipeline, resources};

pub struct Shadow {
    pub view: wgpu::TextureView,
    pub group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub dirty: bool,
    pipeline: wgpu::RenderPipeline,
    origin: wgpu::Buffer,
    depth_group: wgpu::BindGroup,
    center: [f32; 4],
}

impl Shadow {
    pub fn new(device: &wgpu::Device) -> Self {
        let pack = resources::create(device);
        let pipeline = pipeline::create(device, &pack.depth_layout);
        Self {
            view: pack.view,
            group: pack.group,
            layout: pack.layout,
            dirty: true,
            pipeline,
            origin: pack.origin,
            depth_group: pack.depth_group,
            center: [0.0; 4],
        }
    }

    pub fn anchor(&mut self, queue: &wgpu::Queue, eye: [f32; 3]) {
        let center = [
            (eye[0] / 16.0).round() * 16.0,
            0.0,
            (eye[2] / 16.0).round() * 16.0,
            0.0,
        ];
        if center != self.center {
            queue.write_buffer(&self.origin, 0, bytemuck::cast_slice(&center));
            self.center = center;
            self.dirty = true;
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
        pass.set_bind_group(0, &self.depth_group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..count, 0, 0..1);
        self.dirty = false;
    }
}
