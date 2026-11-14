use crate::{Demo, capture, snapshot};
use xetal_value::Value;

#[derive(Default)]
pub struct Commands {
    pub prints: Vec<[f32; 6]>,
    pub sound: [f32; 3],
    pub steps: u64,
    pub landings: u64,
    pub wind: f32,
}
impl Demo {
    pub fn commands(&self) -> Result<Commands, String> {
        read(&capture::mesh(&self.captured)?)
    }
}
fn floats(value: &Value<'_>, shape: &[usize]) -> Result<Vec<f32>, String> {
    let Value::Array(array) = value else {
        return Err("expected numeric command array".into());
    };
    if array.shape() != shape {
        return Err("wrong command array shape".into());
    }
    array
        .data()
        .iter()
        .map(|value| {
            let number = snapshot::number(value)? as f32;
            if !number.is_finite() {
                return Err("nonfinite command value".into());
            }
            Ok(number)
        })
        .collect()
}
fn count(value: &Value<'_>) -> Result<u64, String> {
    match value {
        Value::Int(n) if *n >= 0 => Ok(*n as u64),
        _ => Err("invalid event counter".into()),
    }
}
fn read(value: &Value<'_>) -> Result<Commands, String> {
    let Value::Tuple(parts) = value else {
        return Ok(Commands::default());
    };
    if parts.len() != 6 {
        return Err("expected mesh/prints/sound/steps/landings/wind frame".into());
    }
    let Value::Array(prints) = &parts[1] else {
        return Err("expected print array".into());
    };
    let rows = prints
        .shape()
        .first()
        .copied()
        .ok_or("missing print rows")?;
    if rows > 256 {
        return Err("print budget exceeded".into());
    }
    let prints = floats(&parts[1], &[rows, 6])?.as_chunks::<6>().0.to_vec();
    let sound: [f32; 3] = floats(&parts[2], &[3])?
        .try_into()
        .map_err(|_| "invalid sound")?;
    if ![0.0, 1.0, 2.0].contains(&sound[0])
        || !(0.0..=1.0).contains(&sound[1])
        || !(0.1..=4.0).contains(&sound[2])
    {
        return Err("invalid audio command".into());
    }
    Ok(Commands {
        prints,
        sound,
        steps: count(&parts[3])?,
        landings: count(&parts[4])?,
        wind: snapshot::number(&parts[5])? as f32,
    })
}
