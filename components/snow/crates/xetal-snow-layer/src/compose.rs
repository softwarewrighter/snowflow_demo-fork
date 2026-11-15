use crate::{plan::Key, sampling::Coarse, tile::Tile};
use std::{collections::BTreeMap, sync::Arc};
use xetal_world_surface::Mesh;
pub fn mesh(
    base: &Mesh,
    tiles: &mut BTreeMap<Key, Arc<Tile>>,
    coarse: &Coarse,
) -> Result<Mesh, String> {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let keys = tiles.keys().copied().collect();
    for tile in tiles.values_mut() {
        *tile = Arc::new(tile.project(coarse, &keys));
        if tile.heights.iter().any(|y| !y.is_finite()) {
            continue;
        }
        let offset = (positions.len() / 3) as u32;
        for (p, y) in tile.rows.as_chunks::<3>().0.iter().zip(&tile.heights) {
            positions.extend_from_slice(&[p[0], *y, p[2]]);
        }
        for x in 0..tile.n - 1 {
            for z in 0..tile.n - 1 {
                let a = (x * tile.n + z) as u32 + offset;
                let n = tile.n as u32;
                indices.extend_from_slice(&[a, a + 1, a + n, a + 1, a + n + 1, a + n]);
            }
        }
    }
    for triangle in base.positions().as_chunks::<9>().0 {
        let x = (triangle[0] + triangle[3] + triangle[6]) / 3.0;
        let z = (triangle[2] + triangle[5] + triangle[8]) / 3.0;
        if tiles
            .get(&((x / 2.0).floor() as i32, (z / 2.0).floor() as i32))
            .is_some_and(|t| t.heights.iter().all(|y| y.is_finite()))
        {
            continue;
        }
        let offset = (positions.len() / 3) as u32;
        positions.extend_from_slice(triangle);
        indices.extend_from_slice(&[offset, offset + 1, offset + 2]);
    }
    Mesh::from_arrays(
        &[positions.len() / 3, 3],
        positions,
        &[indices.len() / 3, 3],
        indices,
    )
    .map_err(|e| e.to_string())
}
