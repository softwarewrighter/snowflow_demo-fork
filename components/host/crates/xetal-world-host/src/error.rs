use std::fmt;

use crate::MeshHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostError {
    MeshLimit,
    ByteLimit,
    HandleExhausted,
    UnknownHandle(MeshHandle),
}

impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MeshLimit => write!(f, "mesh resource count limit exceeded"),
            Self::ByteLimit => write!(f, "mesh upload byte limit exceeded"),
            Self::HandleExhausted => write!(f, "mesh handle space exhausted; IDs never wrap"),
            Self::UnknownHandle(id) => {
                write!(f, "mesh handle {} is disposed, reset or unknown", id.raw())
            }
        }
    }
}

impl std::error::Error for HostError {}
