use crate::{Renderer, resources};
use xetal_world_surface::Mesh;

impl Renderer {
    pub fn upload(&mut self, mesh: &Mesh) -> Result<(), String> {
        if mesh.byte_len() as u64 > self.device.limits().max_buffer_size {
            return Err("mesh exceeds the device buffer limit".into());
        }
        match self.mesh.as_mut() {
            Some(current)
                if current.bytes == mesh.byte_len()
                    && current.count as usize == mesh.indices().len() =>
            {
                current.update(&self.queue, mesh)
            }
            _ => self.mesh = Some(resources::GpuMesh::new(&self.device, mesh)?),
        }
        self.shadow.dirty = true;
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.config.width == width.max(1) && self.config.height == height.max(1) {
            return;
        }
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.device, &self.config);
        self.depth = resources::depth(&self.device, &self.config);
    }

    pub fn draw(
        &mut self,
        angle: f32,
        tilt: f32,
        eye: [f32; 3],
        effects: [bool; 2],
    ) -> Result<(), String> {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Timeout) => return Ok(()),
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            Err(error) => return Err(error.to_string()),
        };
        let aspect = self.config.width as f32 / self.config.height as f32;
        self.queue.write_buffer(
            &self.uniform,
            0,
            bytemuck::cast_slice(&[
                angle,
                tilt,
                aspect,
                f32::from(u8::from(effects[0])),
                eye[0],
                eye[1],
                eye[2],
                f32::from(u8::from(effects[1])),
            ]),
        );
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        if effects[1]
            && let Some(mesh) = &self.mesh
        {
            self.shadow
                .update(&mut encoder, &mesh.vertices, &mesh.indices, mesh.count);
        }
        self.encode(&mut encoder, &view);
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    fn encode(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let color = color_attachment(view);
        let depth = wgpu::RenderPassDepthStencilAttachment {
            view: &self.depth,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(color)],
            depth_stencil_attachment: Some(depth),
            ..Default::default()
        });
        if let Some(mesh) = &self.mesh {
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.group, &[]);
            pass.set_bind_group(1, &self.shadow.group, &[]);
            pass.set_vertex_buffer(0, mesh.vertices.slice(..));
            pass.set_index_buffer(mesh.indices.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..mesh.count, 0, 0..1);
        }
    }
}

fn color_attachment(view: &wgpu::TextureView) -> wgpu::RenderPassColorAttachment<'_> {
    wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.47,
                g: 0.65,
                b: 0.78,
                a: 1.0,
            }),
            store: wgpu::StoreOp::Store,
        },
    }
}
