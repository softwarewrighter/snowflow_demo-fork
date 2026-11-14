use xetal_value::Value;
use xetal_world_surface::{Mesh, from_scene};

pub(crate) fn mesh(value: &Value<'_>) -> Result<Mesh, String> {
    let value = if let Value::Tuple(fields) = value { fields.first().ok_or("empty frame")? } else { value };
    let Value::Array(array) = value else {
        return Err("mesh! must be a Scene triangle array".into());
    };
    let values = array
        .data()
        .iter()
        .map(number)
        .collect::<Result<Vec<_>, _>>()?;
    from_scene(array.shape(), &values).map_err(|e| e.to_string())
}

pub(crate) fn number(value: &Value<'_>) -> Result<f64, &'static str> {
    match value {
        Value::Float(value) => Ok(*value),
        Value::Int(value) if value.unsigned_abs() <= (1_u64 << 53) => Ok(*value as f64),
        Value::Int(_) => Err("integer coordinate loses Float precision; convert explicitly"),
        _ => Err("mesh! contains a nonnumeric value"),
    }
}
