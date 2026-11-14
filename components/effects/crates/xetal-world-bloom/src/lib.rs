//! Native transport for X_eTaL Bloom geometry, crater and particle commands.
mod output;
mod transport;
pub const SOURCE: &str = include_str!("../../../../world/demos/bloom.xtl");
pub use output::Update;
pub use transport::Bloom;

impl Bloom {
    pub fn targeting(&self) -> [f32; 2] {
        self.targeting
    }
    pub fn particles(&self) -> &[f32] {
        &self.particles
    }
}
