use crate::tile::Tile;
use std::{collections::BTreeMap, sync::Arc};
use xetal_app_session::Detail;
pub type Key = (i32, i32);
pub struct Pending {
    pub signature: u64,
    pub jobs: Vec<(Key, u64)>,
    pub tiles: BTreeMap<Key, Arc<Tile>>,
}
pub fn desired(detail: &Detail, p: [f32; 3]) -> BTreeMap<Key, u64> {
    let mut cells: BTreeMap<Key, Vec<f32>> = BTreeMap::new();
    for (b, row) in detail
        .bounds
        .as_chunks::<4>()
        .0
        .iter()
        .zip(detail.stamps.as_chunks::<6>().0)
    {
        for x in (b[0] / 2.0).floor() as i32..=(b[1] / 2.0).floor() as i32 {
            for z in (b[2] / 2.0).floor() as i32..=(b[3] / 2.0).floor() as i32 {
                if ((x as f32 * 2.0 + 1.0) - p[0]).abs() <= detail.config[2]
                    && ((z as f32 * 2.0 + 1.0) - p[2]).abs() <= detail.config[2]
                {
                    cells.entry((x, z)).or_default().extend_from_slice(row);
                }
            }
        }
    }
    let mut cells = cells.into_iter().collect::<Vec<_>>();
    cells.sort_by(|(a, _), (b, _)| {
        let distance = |k: Key| (k.0 as f32 * 2.0 - p[0]).hypot(k.1 as f32 * 2.0 - p[2]);
        distance(*a).total_cmp(&distance(*b))
    });
    cells
        .into_iter()
        .take(detail.config[3] as usize)
        .map(|(k, rows)| {
            (
                k,
                hash(
                    std::iter::once(detail.config[0].to_bits() as u64)
                        .chain(rows.iter().map(|n| n.to_bits() as u64)),
                ),
            )
        })
        .collect()
}
pub fn hash(values: impl IntoIterator<Item = u64>) -> u64 {
    values.into_iter().fold(14695981039346656037, |h, n| {
        (h ^ n).wrapping_mul(1099511628211)
    })
}
pub fn signature(desired: &BTreeMap<Key, u64>, config: [f32; 4]) -> u64 {
    hash(
        config.iter().map(|n| n.to_bits() as u64).chain(
            desired
                .iter()
                .flat_map(|(k, h)| [k.0 as u64, k.1 as u64, *h]),
        ),
    )
}
pub fn pending(
    desired: BTreeMap<Key, u64>,
    signature: u64,
    current: &BTreeMap<Key, Arc<Tile>>,
) -> Pending {
    let mut p = Pending {
        signature,
        jobs: Vec::new(),
        tiles: BTreeMap::new(),
    };
    for (key, hash) in desired {
        if let Some(tile) = current.get(&key).filter(|t| t.hash == hash) {
            p.tiles.insert(key, tile.clone());
        } else {
            p.jobs.push((key, hash));
        }
    }
    p
}
