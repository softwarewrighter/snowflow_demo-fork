use xetal_world_model::Demo;
use xetal_world_surface::Mesh;
#[derive(Default)]
pub struct Update {
    pub mesh: Option<Mesh>,
    pub score: Option<[f32; 4]>,
}
#[derive(Default)]
pub(crate) struct Cache {
    pub positions: Vec<f32>,
    pub mode: u8,
    pub tip: [f32; 3],
    score: [f32; 4],
}
fn numeric(demo: &Demo, slot: usize, count: usize) -> Result<Vec<f32>, String> {
    let (shape, data) = demo.array(slot)?;
    if shape != [count] {
        return Err("invalid Ribbon publication shape".into());
    }
    let values: Vec<_> = data.iter().map(|v| *v as f32).collect();
    if values.iter().any(|v| !v.is_finite()) {
        return Err("nonfinite Ribbon publication".into());
    }
    Ok(values)
}
pub(crate) fn probe(demo: &Demo) -> Result<Option<[f32; 3]>, String> {
    let values = numeric(demo, 2, 4)?;
    if ![0.0, 1.0].contains(&values[0]) {
        return Err("invalid Ribbon probe state".into());
    }
    Ok((values[0] == 1.0).then_some([values[1], values[2], values[3]]))
}
pub(crate) fn read(demo: &Demo, cache: &mut Cache) -> Result<Update, String> {
    let values = numeric(demo, 1, 4)?;
    let score = [values[0], values[1], values[2], values[3]];
    if score[0] < 0.0 || score[0].fract() != 0.0 || score[3] <= 0.0 || score[3] > 4.0 {
        return Err("invalid Ribbon score command".into());
    }
    let command = (score[0] > 0.0 && score != cache.score).then_some(score);
    cache.score = score;
    let status = numeric(demo, 3, 2)?;
    if ![0.0, 1.0, 2.0, 3.0].contains(&status[0]) || !(0.0..=32.0).contains(&status[1]) {
        return Err("invalid Ribbon state/history budget".into());
    }
    cache.mode = status[0] as u8;
    let tip = numeric(demo, 2, 4)?;
    cache.tip = [tip[1], tip[2], tip[3]];
    let mesh = demo.mesh()?;
    if mesh.indices().len() > 496 * 3 {
        return Err("Ribbon geometry budget exceeded".into());
    }
    let mesh = if mesh.positions() != cache.positions {
        cache.positions = mesh.positions().to_vec();
        Some(mesh)
    } else {
        None
    };
    Ok(Update {
        mesh,
        score: command,
    })
}
