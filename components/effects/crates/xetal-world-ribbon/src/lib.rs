//! Native transport for pure X_eTaL Ribbon path-history geometry and scoring.
mod input;
mod io;
mod output;
mod transport;
pub const SOURCE: &str = include_str!("../../../../world/demos/ribbon.xtl");
pub use output::Update;
pub use transport::Ribbon;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub origin: [f32; 3],
    pub aim: [f32; 2],
}
impl Pose {
    pub(crate) fn validate(self) -> Result<(), String> {
        if self.origin.iter().chain(&self.aim).any(|v| !v.is_finite()) {
            return Err("nonfinite Ribbon pose".into());
        }
        Ok(())
    }
}
impl Ribbon {
    pub fn mode(&self) -> u8 {
        self.cache.mode
    }
    pub fn tip(&self) -> [f32; 3] {
        self.cache.tip
    }
}
