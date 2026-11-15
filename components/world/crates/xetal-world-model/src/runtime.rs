use crate::{SOURCE, capture, snapshot};
use std::cell::RefCell;
use std::rc::Rc;
use xetal_arith::Rng;
use xetal_snow_libraries::Embedded;
use xetal_step::{Machine, Status};
use xetal_world_surface::Mesh;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Progress {
    Running,
    Waiting,
    Done,
}

pub struct Demo {
    machine: Machine<'static, 'static>,
    pub(crate) captured: capture::Captured,
    progress: Progress,
}

impl Demo {
    pub fn new() -> Result<Self, String> {
        Self::from_source(SOURCE)
    }

    pub fn from_source(source: &str) -> Result<Self, String> {
        let mut loaded = xetal_program::load_with("snowfield.xtl", source, &Embedded)
            .map_err(|e| e.to_string())?;
        xetal_types::check_program(&mut loaded.program)
            .map_err(|e| xetal_program::located(&loaded.sources, e).to_string())?;
        let loaded = Box::leak(Box::new(loaded));
        let captured = Rc::new(RefCell::new(None));
        let output = Box::leak(Box::new(std::io::sink()));
        let machine = Machine::new(&loaded.program, output, Rng::seeded(1))
            .hooks(Some(capture::keeper(captured.clone())), None)
            .waiting_for_input();
        Ok(Self {
            machine,
            captured,
            progress: Progress::Running,
        })
    }

    pub fn step(&mut self, budget: usize) -> Result<Progress, String> {
        self.progress = match self.machine.run(budget).map_err(|e| e.to_string())? {
            Status::Running => Progress::Running,
            Status::WaitingEvent => Progress::Waiting,
            Status::Done => Progress::Done,
            _ => return Err("snowfield requested unsupported input".into()),
        };
        Ok(self.progress)
    }

    pub fn event(&mut self, event: &str) -> Result<(), String> {
        if self.progress != Progress::Waiting {
            return Err("program is not waiting for an event".into());
        }
        self.machine.feed(event.into());
        self.progress = Progress::Running;
        Ok(())
    }

    /// Read a numeric array from a typed publication tuple, without text serialization.
    pub fn array(&self, index: usize) -> Result<(Vec<usize>, Vec<f64>), String> {
        let frame = capture::mesh(&self.captured)?;
        let xetal_value::Value::Tuple(parts) = frame else {
            return Err("expected typed publication tuple".into());
        };
        snapshot::array(parts.get(index).ok_or("missing publication field")?)
    }

    /// Clone the typed application publication without text serialization.
    pub fn publication(&self) -> Result<xetal_value::Value<'static>, String> {
        capture::mesh(&self.captured)
    }

    pub fn mesh(&self) -> Result<Mesh, String> {
        snapshot::mesh(&capture::mesh(&self.captured)?)
    }
}
