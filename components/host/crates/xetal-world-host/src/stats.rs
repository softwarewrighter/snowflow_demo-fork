use crate::MeshStore;

#[derive(Debug, Copy, Clone)]
pub struct Limits {
    pub max_meshes: usize,
    pub max_bytes: usize,
}

/// Live logical upload bytes; not a measurement of allocator or GPU memory.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Stats {
    pub meshes: usize,
    pub bytes: usize,
}

impl MeshStore {
    pub fn stats(&self) -> Stats {
        Stats {
            meshes: self.meshes.len(),
            bytes: self.bytes,
        }
    }
}
