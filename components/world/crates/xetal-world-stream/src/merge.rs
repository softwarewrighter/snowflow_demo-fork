use crate::Key;
use std::collections::BTreeMap;
use xetal_world_surface::Mesh;

pub(crate) fn mesh(chunks: &BTreeMap<Key, Mesh>) -> Result<Mesh, String> {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    for chunk in chunks.values() {
        let offset =
            u32::try_from(positions.len() / 3).map_err(|_| "too many streamed vertices")?;
        positions.extend_from_slice(chunk.positions());
        indices.extend(chunk.indices().iter().map(|index| index + offset));
    }
    Mesh::from_arrays(
        &[positions.len() / 3, 3],
        positions,
        &[indices.len() / 3, 3],
        indices,
    )
    .map_err(|error| error.to_string())
}
