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
    let mut frame = Frame {
        resources: Vec::new(),
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
    };
    validate(&frame)?;
    frame.resources = cache.read(&f[1])?;
    Ok(frame)
}
fn validate(frame: &Frame) -> Result<(), String> {
    let [kind, gain, rate] = frame.audio;
    if ![0.0, 1.0, 2.0].contains(&kind)
        || !(0.0..=1.0).contains(&gain)
        || !(0.1..=4.0).contains(&rate)
    {
        return Err("invalid audio command".into());
    }
    let n = frame.navigation;
    if ![0.0, 1.0].contains(&n[0])
        || n[1].abs() > 1.0
        || n[2].abs() > 1.0
        || n[5].abs() > 1.2
        || !(n[4] as f32).is_finite()
    {
        return Err("invalid navigation".into());
    }
    for i in [3, 6, 7] {
        if n[i] < 0.0 || n[i] > (1_u64 << 53) as f64 || n[i].fract() != 0.0 {
            return Err("invalid navigation sequence".into());
        }
    }
    queries(&frame.requests)?;
    for p in frame.particles.as_chunks::<6>().0 {
        if p[3] <= 0.0 || p[3] > 2.0 || !(0.0..=1.0).contains(&p[4]) {
            return Err("invalid particle size/opacity".into());
        }
    }
    for p in frame.prints.as_chunks::<6>().0 {
        if ![-1.0, 1.0].contains(&p[3]) || p[4] < 0.0 || !(0.0..=1.0).contains(&p[5]) {
            return Err("invalid footprint command".into());
        }
    }
    if frame
        .dirty
        .as_chunks::<4>()
        .0
        .iter()
        .any(|b| b[0] > b[1] || b[2] > b[3])
    {
        return Err("invalid dirty bounds".into());
    }
    Ok(())
}

fn queries(rows: &[f32]) -> Result<(), String> {
    for q in rows.as_chunks::<10>().0 {
        if q[0] <= 0.0
            || q[0].fract() != 0.0
            || q[0] as f64 > u32::MAX as f64
            || ![1.0, 2.0].contains(&q[1])
        {
            return Err("invalid physics query".into());
        }
        if q[1] == 1.0 && (!(0.0..=128.0).contains(&q[8]) || !(0.0..=128.0).contains(&q[9])) {
            return Err("invalid query range".into());
        }
    }
    Ok(())
}
