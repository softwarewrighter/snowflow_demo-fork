use crate::{Button, Cache, Resource, Source, metadata, value::*};
use std::collections::BTreeMap;
use xetal_value::Value;
#[derive(Clone)]
pub struct Frame {
    pub resources: Vec<Resource>,
    pub particles_revision: u64,
    pub particles: Vec<f32>,
    pub prints: Vec<f32>,
    pub audio_sequence: u64,
    pub audio: [f32; 3],
    pub stats: BTreeMap<String, f64>,
    pub requests: Vec<f32>,
    pub buttons: Vec<Button>,
    pub navigation: [f64; 8],
    pub world_revision: u64,
    pub dirty: Vec<f32>,
    pub sources: Vec<Source>,
}
pub fn decode(v: &Value<'_>, cache: &mut Cache) -> Result<Frame, String> {
    let f = tuple(v, 12)?;
    if sequence(&f[11])? != 1 {
        return Err("unsupported application protocol".into());
    }
    let particles = tuple(&f[2], 2)?;
    let audio = tuple(&f[4], 2)?;
    let Value::Array(sound) = &audio[1] else {
        return Err("expected audio command".into());
    };
    if sound.shape() != [3] {
        return Err("invalid audio command shape".into());
    }
    let sound = sound
        .data()
        .iter()
        .map(number)
        .collect::<Result<Vec<_>, _>>()?;
    let nav = tuple(&f[8], 8)?
        .iter()
        .map(number)
        .collect::<Result<Vec<_>, _>>()?;
    let world = tuple(&f[9], 2)?;
    let mut stats = BTreeMap::new();
    metadata::stats(&f[5], &mut stats)?;
    let mut buttons = Vec::new();
    metadata::buttons(&f[7], &mut buttons)?;
    Ok(Frame {
        resources: cache.read(&f[1])?,
        particles_revision: sequence(&particles[0])?,
        particles: array(&particles[1], 6, 1024)?,
        prints: array(&f[3], 6, 256)?,
        audio_sequence: sequence(&audio[0])?,
        audio: [sound[0] as f32, sound[1] as f32, sound[2] as f32],
        stats,
        requests: array(&f[6], 10, 16)?,
        buttons,
        navigation: nav.try_into().map_err(|_| "invalid navigation")?,
        world_revision: sequence(&world[0])?,
        dirty: array(&world[1], 4, 64)?,
        sources: metadata::sources(&f[10])?,
    })
}
