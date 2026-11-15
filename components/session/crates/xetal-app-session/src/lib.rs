//! One persistent X_eTaL application, with generic input and capability transport.
mod session;
mod transport;
pub use session::Session;
pub use xetal_app_protocol::{Button, Frame, Resource, Source};
pub const SOURCE: &str = include_str!("../../../../world/demos/App.xtl");
#[derive(Clone, Copy, Debug)]
pub struct Pose {
    pub position: [f32; 3],
    pub aim: [f32; 2],
    pub grounded: bool,
    pub impact: f32,
}
