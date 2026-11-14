use std::sync::atomic::{AtomicU32, Ordering};

use crate::HostError;

static NEXT: AtomicU32 = AtomicU32::new(1);

/// Execution-instance mesh token. Never a vertex, material or transform-group ID.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MeshHandle(u32);

impl MeshHandle {
    pub fn raw(self) -> u32 {
        self.0
    }
}

pub(crate) fn allocate() -> Result<MeshHandle, HostError> {
    NEXT.try_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .map(MeshHandle)
        .map_err(|_| HostError::HandleExhausted)
}
