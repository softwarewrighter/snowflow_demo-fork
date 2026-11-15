use crate::{Pose, SOURCE};
use xetal_app_protocol::{Cache, Frame, decode};
use xetal_world_model::Demo;
use xetal_world_surface::Mesh;
pub struct Session {
    pub(crate) demo: Demo,
    cache: Cache,
    publication: Option<xetal_value::Value<'static>>,
    frame: Option<Frame>,
}
impl Session {
    pub fn new() -> Result<Self, String> {
        Self::from_source(SOURCE)
    }
    pub fn from_source(source: &str) -> Result<Self, String> {
        let mut app = Self {
            demo: Demo::from_source(source)?,
            cache: Cache::default(),
            publication: None,
            frame: None,
        };
        app.settle()?;
        Ok(app)
    }
    pub fn read(&mut self) -> Result<Frame, String> {
        let next = self.demo.publication()?;
        if let (Some(xetal_value::Value::Tuple(old)), xetal_value::Value::Tuple(new), Some(frame)) =
            (&self.publication, &next, &self.frame)
            && std::rc::Rc::ptr_eq(old, new)
        {
            return Ok(frame.clone());
        }
        let frame = decode(&next, &mut self.cache)?;
        self.publication = Some(next);
        self.frame = Some(frame.clone());
        Ok(frame)
    }
    pub fn tick(&mut self, dt: f32) -> Result<(), String> {
        if !dt.is_finite() || !(0.0..=0.25).contains(&dt) {
            return Err("invalid tick duration".into());
        }
        self.send(&format!("tick {dt}"))
    }
    pub fn input(&mut self, key: &str, pressed: bool) -> Result<(), String> {
        let key = key.strip_prefix("Arrow").unwrap_or(key);
        let key = if key == " " || key == "Space" {
            "Space".into()
        } else {
            key.to_ascii_lowercase()
        };
        if !((key.len() == 1 && key.as_bytes()[0].is_ascii_graphic())
            || [
                "Space",
                "up",
                "down",
                "left",
                "right",
                "home",
                "end",
                "escape",
                "enter",
                "tab",
                "delete",
                "backspace",
            ]
            .contains(&key.as_str()))
        {
            return Err("unsupported input key".into());
        }
        self.send(&format!("choose {} 0", u8::from(pressed)))?;
        self.send(&format!("key {key}"))
    }
    pub fn pose(&mut self, pose: Pose) -> Result<(), String> {
        if pose
            .position
            .iter()
            .chain(&pose.aim)
            .chain([&pose.impact])
            .any(|n| !n.is_finite())
        {
            return Err("nonfinite pose".into());
        }
        self.send(&format!("move {} {}", pose.position[0], pose.position[2]))?;
        self.send(&format!("down {} {}", pose.position[1], pose.impact))?;
        self.send(&format!("up {} {}", pose.aim[0], pose.aim[1]))?;
        self.send(&format!("choose {} 0", u8::from(pose.grounded)))?;
        self.send("click 1 0")
    }
    pub fn chunk(&mut self, key: (i32, i32)) -> Result<Mesh, String> {
        self.send(&format!("move {} {}", key.0, key.1))?;
        self.send("click 2 0")?;
        self.demo.mesh()
    }
}
