use crate::{Mesh, MeshError, validate};

/// Adapt Scene's T by 3 by 3 triangles without changing vertex order or winding.
pub fn from_scene(shape: &[usize], triangles: &[f64]) -> Result<Mesh, MeshError> {
    let count = validate::shape(shape, &[3, 3], triangles.len(), "scene triangles")?;
    let vertices = triangles.len() / 3;
    let end = u32::try_from(vertices).map_err(|_| MeshError::VertexCount)?;
    Mesh::from_arrays(
        &[vertices, 3],
        validate::narrow(triangles)?,
        &[count, 3],
        (0..end).collect(),
    )
}

/// Adapt N by 3 Floats and T by 3 one-based Int indices to the application payload.
pub fn from_xetal(
    position_shape: &[usize],
    positions: &[f64],
    index_shape: &[usize],
    indices: &[i64],
) -> Result<Mesh, MeshError> {
    let vertices = validate::shape(position_shape, &[3], positions.len(), "positions")?;
    validate::shape(index_shape, &[3], indices.len(), "indices")?;
    u32::try_from(vertices).map_err(|_| MeshError::VertexCount)?;
    let indices = validate::one_based(vertices, indices)?;
    Mesh::from_arrays(
        position_shape,
        validate::narrow(positions)?,
        index_shape,
        indices,
    )
}
