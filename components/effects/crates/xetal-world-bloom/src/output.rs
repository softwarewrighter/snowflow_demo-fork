use xetal_world_model::Demo;
use xetal_world_surface::Mesh;
#[derive(Default)]
pub struct Update {
    pub mesh: Option<Mesh>,
    pub particles: Option<Vec<f32>>,
    pub crater: Option<[f32; 3]>,
}
pub(crate) fn targeting(demo: &Demo) -> Result<[f32; 2], String> {
    let values = numeric(demo, 3, &[2])?;
    if values.iter().any(|v| *v <= 0.0 || *v > 100.0) {
        return Err("invalid Bloom targeting limits".into());
    }
    Ok([values[0], values[1]])
}
fn numeric(demo: &Demo, index: usize, shape: &[usize]) -> Result<Vec<f32>, String> {
    let (dims, values) = demo.array(index)?;
    if dims != shape {
        return Err("invalid Bloom output shape".into());
    }
    let values: Vec<_> = values.iter().map(|v| *v as f32).collect();
    if values.iter().any(|v| !v.is_finite()) {
        return Err("invalid Bloom output values".into());
    }
    Ok(values)
}
pub(crate) fn read(
    demo: &Demo,
    positions: &mut Vec<f32>,
    particles: &mut Vec<f32>,
    last: &mut [f32; 3],
) -> Result<Update, String> {
    let values = numeric(demo, 1, &[3])?;
    let crater = [values[0], values[1], values[2]];
    if crater[0] < 0.0 || crater[0].fract() != 0.0 {
        return Err("invalid crater serial".into());
    }
    let command = (crater[0] > 0.0 && crater != *last).then_some(crater);
    *last = crater;
    let (shape, _) = demo.array(2)?;
    if shape.len() != 2 || shape[0] > 512 || shape[1] != 6 {
        return Err("particle budget or shape exceeded".into());
    }
    let rows = numeric(demo, 2, &shape)?;
    if rows
        .as_chunks::<6>()
        .0
        .iter()
        .any(|p| p[3] <= 0.0 || !(0.0..=1.0).contains(&p[4]))
    {
        return Err("invalid particle size or opacity".into());
    }
    let changed = (rows != *particles).then(|| rows.clone());
    *particles = rows;
    let mesh = demo.mesh()?;
    if mesh.indices().len() > 480 * 3 {
        return Err("Bloom mesh budget exceeded".into());
    }
    let mesh = if mesh.positions() != positions {
        *positions = mesh.positions().to_vec();
        Some(mesh)
    } else {
        None
    };
    Ok(Update {
        mesh,
        particles: changed,
        crater: command,
    })
}
