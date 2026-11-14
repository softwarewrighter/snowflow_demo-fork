use std::fmt;

/// Invalid application mesh data; no input is repaired implicitly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeshError {
    Shape(&'static str),
    Length {
        array: &'static str,
        expected: usize,
        got: usize,
    },
    NonFinite(usize),
    Narrowing(usize),
    Index {
        offset: usize,
        index: i64,
        vertices: usize,
    },
    VertexCount,
}

impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shape(name) => write!(f, "{name}: invalid or overflowing shape"),
            Self::Length {
                array,
                expected,
                got,
            } => write!(f, "{array}: expected {expected} values, got {got}"),
            Self::NonFinite(offset) => write!(f, "positions: nonfinite value at offset {offset}"),
            Self::Narrowing(offset) => {
                write!(f, "positions: f32 overflow or underflow at offset {offset}")
            }
            Self::Index {
                offset,
                index,
                vertices,
            } => write!(
                f,
                "indices: {index} at offset {offset} is outside {vertices} vertices"
            ),
            Self::VertexCount => write!(f, "vertex count exceeds the u32 mesh index limit"),
        }
    }
}

impl std::error::Error for MeshError {}
