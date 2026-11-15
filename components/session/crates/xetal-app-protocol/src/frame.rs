use crate::{Button, Cache, Resource, Source, metadata, value::*};
use std::collections::BTreeMap;
use xetal_value::Value;
#[derive(Clone)]
pub struct Frame {
    pub detail: Detail,
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
#[derive(Clone)]
pub struct Detail {
    pub config: [f32; 4],
    pub bounds: Vec<f32>,
    pub stamps: Vec<f32>,
}
fn detail(value: &Value<'_>) -> Result<Detail, String> {
    let f = tuple(value, 3)?;
    let Value::Array(config) = &f[0] else {
        return Err("invalid grid config".into());
    };
    if config.shape() != [4] {
        return Err("invalid grid config shape".into());
    }
    let values = config
        .data()
        .iter()
        .map(number)
        .collect::<Result<Vec<_>, _>>()?;
    let c = values.iter().map(|n| *n as f32).collect::<Vec<_>>();
    if !(0.05..=0.1).contains(&c[0])
        || c[1] != 2.0
        || !(2.0..=8.0).contains(&c[2])
        || !(1.0..=36.0).contains(&c[3])
        || c[3].fract() != 0.0
    {
        return Err("detail grid budget exceeded".into());
    }
    if ((2.0 / c[0]) - (2.0 / c[0]).round()).abs() > 0.0001 {
        return Err("detail spacing must divide the tile".into());
    }
    let bounds = array(&f[1], 4, 256)?;
    let stamps = array(&f[2], 6, 256)?;
    if bounds.len() / 4 != stamps.len() / 6 {
        return Err("detail bounds/stamps mismatch".into());
    }
    if bounds.as_chunks::<4>().0.iter().any(|b| {
        b[0] > b[1]
            || b[2] > b[3]
            || b[1] - b[0] > 2.0
            || b[3] - b[2] > 2.0
            || b.iter().any(|v| v.abs() > 10000000.0)
    }) {
        return Err("invalid detail bounds".into());
    }
    Ok(Detail {
        config: c.try_into().map_err(|_| "invalid grid config")?,
        bounds,
        stamps,
    })
}
pub fn decode(v: &Value<'_>, cache: &mut Cache) -> Result<Frame, String> {
    let f = tuple(v, 12)?;
    if sequence(&f[11])? != 2 {
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
    let world = tuple(&f[9], 3)?;
    let mut stats = BTreeMap::new();
    metadata::stats(&f[5], &mut stats)?;
    let mut buttons = Vec::new();
    metadata::buttons(&f[7], &mut buttons)?;
    let mut frame = Frame {
        detail: detail(&world[2])?,
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
