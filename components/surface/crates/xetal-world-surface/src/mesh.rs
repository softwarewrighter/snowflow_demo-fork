use crate::{MeshError, validate};

/// Finite xyz positions and zero-based triangle indices, stored without repacking.
#[derive(Debug, PartialEq)]
pub struct Mesh {
    positions: Vec<f32>,
    indices: Vec<u32>,
}

impl Mesh {
    /// Consume N by 3 positions and T by 3 indices. Empty meshes are valid.
    pub fn from_arrays(
        position_shape: &[usize],
        positions: Vec<f32>,
        index_shape: &[usize],
        indices: Vec<u32>,
    ) -> Result<Self, MeshError> {
        let vertices = validate::shape(position_shape, &[3], positions.len(), "positions")?;
        validate::shape(index_shape, &[3], indices.len(), "indices")?;
        validate::contents(vertices, &positions, &indices)?;
        Ok(Self { positions, indices })
    }

    pub fn positions(&self) -> &[f32] {
        &self.positions
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Logical upload bytes; excludes allocator capacity and GPU overhead.
    pub fn byte_len(&self) -> usize {
        (self.positions.len() + self.indices.len()) * 4
    }
}
