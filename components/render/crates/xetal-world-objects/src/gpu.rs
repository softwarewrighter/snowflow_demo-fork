pub struct Objects {
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
    count: u32,
}
impl Objects {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        camera: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            pipeline: crate::pipeline::create(device, format, camera),
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("bounded procedural ice"),
                size: (crate::MAX_VERTICES * 3 * 4) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            count: 0,
        }
    }
    pub fn update(&mut self, queue: &wgpu::Queue, vertices: &[f32]) -> Result<(), String> {
        crate::validate(vertices)?;
        self.count = (vertices.len() / 3) as u32;
        if !vertices.is_empty() {
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(vertices));
        }
        Ok(())
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(0..self.count, 0..1);
    }
}
