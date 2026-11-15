use crate::value::{number, text, tuple};
use std::collections::BTreeMap;
use xetal_value::Value;
#[derive(Clone, Debug)]
pub struct Button {
    pub id: String,
    pub key: String,
    pub label: String,
    pub held: bool,
    pub active: bool,
}
#[derive(Clone, Debug)]
pub struct Source {
    pub id: String,
    pub title: String,
    pub name: String,
}
pub(crate) fn buttons(v: &Value<'_>, out: &mut Vec<Button>) -> Result<(), String> {
    let Value::Tuple(rows) = v else {
        return Err("expected buttons tuple".into());
    };
    if rows.len() == 5 && text(&rows[0]).is_ok() {
        if out.len() == 32 {
            return Err("button budget exceeded".into());
        }
        out.push(Button {
            id: text(&rows[0])?,
            key: text(&rows[1])?,
            label: text(&rows[2])?,
            held: number(&rows[3])? == 1.0,
            active: number(&rows[4])? == 1.0,
        });
    } else {
        for row in rows.iter() {
            buttons(row, out)?;
        }
    }
    Ok(())
}
pub(crate) fn stats(v: &Value<'_>, out: &mut BTreeMap<String, f64>) -> Result<(), String> {
    let Value::Tuple(rows) = v else {
        return Err("expected stats tuple".into());
    };
    if rows.len() == 2 && text(&rows[0]).is_ok() {
        if out.len() == 64 {
            return Err("stats budget exceeded".into());
        }
        out.insert(text(&rows[0])?, number(&rows[1])?);
    } else {
        for row in rows.iter() {
            stats(row, out)?;
        }
    }
    Ok(())
}
pub(crate) fn sources(v: &Value<'_>) -> Result<Vec<Source>, String> {
    let Value::Tuple(rows) = v else {
        return Err("expected sources tuple".into());
    };
    if rows.len() > 16 {
        return Err("source budget exceeded".into());
    }
    rows.iter()
        .map(|row| {
            let f = tuple(row, 3)?;
            Ok(Source {
                id: text(&f[0])?,
                title: text(&f[1])?,
                name: text(&f[2])?,
            })
        })
        .collect()
}
