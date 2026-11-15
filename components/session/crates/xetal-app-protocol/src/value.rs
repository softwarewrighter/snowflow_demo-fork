use xetal_value::Value;
pub(crate) fn tuple<'a, 'b>(v: &'a Value<'b>, n: usize) -> Result<&'a [Value<'b>], String> {
    match v {
        Value::Tuple(v) if v.len() == n => Ok(v),
        _ => Err(format!("expected tuple of {n}")),
    }
}
pub(crate) fn number(v: &Value<'_>) -> Result<f64, String> {
    let n = match v {
        Value::Bool(n) => f64::from(u8::from(*n)),
        Value::Int(n) if n.unsigned_abs() <= 1 << 53 => *n as f64,
        Value::Float(n) => *n,
        _ => return Err("expected precise number".into()),
    };
    if n.is_finite() {
        Ok(n)
    } else {
        Err("nonfinite number".into())
    }
}
pub(crate) fn sequence(v: &Value<'_>) -> Result<u64, String> {
    let n = number(v)?;
    if n >= 0.0 && n.fract() == 0.0 {
        Ok(n as u64)
    } else {
        Err("invalid sequence".into())
    }
}
pub(crate) fn text(v: &Value<'_>) -> Result<String, String> {
    let Value::Array(a) = v else {
        return Err("expected string".into());
    };
    if a.shape().len() != 1 || a.data().len() > 256 {
        return Err("invalid string length".into());
    }
    a.data()
        .iter()
        .map(|v| match v {
            Value::Char(c) => Ok(*c),
            _ => Err("expected character".into()),
        })
        .collect()
}
pub(crate) fn array(v: &Value<'_>, width: usize, limit: usize) -> Result<Vec<f32>, String> {
    let Value::Array(a) = v else {
        return Err("expected numeric array".into());
    };
    if a.data().len() > limit * width || a.shape() != [a.data().len() / width, width] {
        return Err("invalid array shape/budget".into());
    }
    a.data()
        .iter()
        .map(|v| {
            let n = number(v)? as f32;
            if n.is_finite() {
                Ok(n)
            } else {
                Err("Float32 overflow".into())
            }
        })
        .collect()
}
