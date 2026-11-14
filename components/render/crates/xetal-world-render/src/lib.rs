//! Browser WebGPU renderer for validated application mesh payloads.
pub use xetal_world_surface::Mesh;
#[cfg(target_arch = "wasm32")]
mod device;
#[cfg(target_arch = "wasm32")]
mod frame;
#[cfg(target_arch = "wasm32")]
mod pipeline;
#[cfg(target_arch = "wasm32")]
mod resources;
#[cfg(target_arch = "wasm32")]
pub use device::Renderer;

#[cfg(target_arch = "wasm32")]
impl Renderer {
    pub fn upload_ribbon(&mut self, vertices: &[f32]) -> Result<(), String> {
        self.ribbon.update(&self.queue, vertices)
    }
    pub fn upload_particles(&mut self, rows: &[f32]) -> Result<(), String> {
        self.particles.update(&self.queue, rows)
    }
    pub fn upload_sweep(&mut self, vertices: &[f32]) -> Result<(), String> {
        self.slush.update(&self.queue, vertices)
    }
    pub fn upload_objects(&mut self, vertices: &[f32]) -> Result<(), String> {
        self.objects.update(&self.queue, vertices)
    }
    pub fn upload_tracks(&mut self, vertices: &[f32]) -> Result<(), String> {
        self.decals.update(&self.queue, vertices)
    }
}
