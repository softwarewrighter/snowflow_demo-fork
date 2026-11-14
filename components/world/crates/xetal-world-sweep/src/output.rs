use xetal_world_model::Demo;
use xetal_world_surface::Mesh;
#[derive(Default)]
pub struct Update {
    pub mesh: Option<Mesh>,
    pub brush: Option<[f32; 5]>,
}
pub(crate) fn read(
    demo: &Demo,
    positions: &mut Vec<f32>,
    last: &mut [f32; 5],
) -> Result<Update, String> {
    let (shape, values) = demo.array(1)?;
    if shape != [5] {
        return Err("invalid Sweep brush command shape".into());
    }
    let brush: [f32; 5] = values
        .iter()
        .map(|v| *v as f32)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| "invalid brush command")?;
    if brush.iter().any(|v| !v.is_finite()) || brush[0] < 0.0 || !(0.0..=15.0).contains(&brush[4]) {
        return Err("invalid Sweep brush command".into());
    }
    let command = (brush[0] > 0.0 && brush != *last).then_some(brush);
    *last = brush;
    let mesh = demo.mesh()?;
    if mesh.indices().len() > 384 * 3 {
        return Err("Sweep mesh exceeds resource budget".into());
    }
    if mesh.positions() == positions {
        return Ok(Update {
            mesh: None,
            brush: command,
        });
    }
    *positions = mesh.positions().to_vec();
    Ok(Update {
        mesh: Some(mesh),
        brush: command,
    })
}
