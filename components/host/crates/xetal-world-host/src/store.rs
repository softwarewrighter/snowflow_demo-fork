use std::collections::HashMap;

use xetal_world_surface::Mesh;

use crate::{HostError, Limits, MeshHandle, handle};

pub struct MeshStore {
    pub(crate) meshes: HashMap<MeshHandle, Mesh>,
    pub(crate) bytes: usize,
    limits: Limits,
}

impl MeshStore {
    pub fn new(limits: Limits) -> Self {
        Self {
            meshes: HashMap::new(),
            bytes: 0,
            limits,
        }
    }

    /// Retain owned buffers without text conversion; failure preserves store state.
    pub fn insert(&mut self, mesh: Mesh) -> Result<MeshHandle, HostError> {
        if self.meshes.len() >= self.limits.max_meshes {
            return Err(HostError::MeshLimit);
        }
        if mesh.byte_len() > self.limits.max_bytes.saturating_sub(self.bytes) {
            return Err(HostError::ByteLimit);
        }
        let id = handle::allocate()?;
        self.bytes += mesh.byte_len();
        self.meshes.insert(id, mesh);
        Ok(id)
    }

    pub fn get(&self, id: MeshHandle) -> Result<&Mesh, HostError> {
        self.meshes.get(&id).ok_or(HostError::UnknownHandle(id))
    }

    pub fn dispose(&mut self, id: MeshHandle) -> Result<(), HostError> {
        let mesh = self
            .meshes
            .remove(&id)
            .ok_or(HostError::UnknownHandle(id))?;
        self.bytes -= mesh.byte_len();
        Ok(())
    }

    pub fn reset(&mut self) {
        self.meshes.clear();
        self.bytes = 0;
    }
}
