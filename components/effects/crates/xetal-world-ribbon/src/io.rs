use xetal_world_model::{Demo, Progress};
pub(crate) fn settle(demo: &mut Demo) -> Result<(), String> {
    for _ in 0..100 {
        match demo.step(8_000)? {
            Progress::Waiting => return Ok(()),
            Progress::Running => {}
            Progress::Done => return Err("Ribbon app ended".into()),
        }
    }
    Err("Ribbon app exceeded its execution budget".into())
}
pub(crate) fn send(demo: &mut Demo, event: &str) -> Result<(), String> {
    demo.event(event)?;
    settle(demo)
}
