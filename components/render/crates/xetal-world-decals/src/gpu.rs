pub struct Decals {
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
    count: u32,
}
impl Decals {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        camera: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            pipeline: crate::pipeline::create(device, format, camera),
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("bounded snow tracks"),
                size: 256 * 6 * 5 * 4,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            count: 0,
        }
    }
    pub fn update(&mut self, queue: &wgpu::Queue, vertices: &[f32]) -> Result<(), String> {
        if vertices.len() > 256 * 6 * 5 || !vertices.len().is_multiple_of(30) {
            return Err("invalid footprint vertex payload".into());
        }
        self.count = (vertices.len() / 5) as u32;
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
