use crate::MeshError;

pub(crate) fn shape(
    dims: &[usize],
    tail: &[usize],
    len: usize,
    name: &'static str,
) -> Result<usize, MeshError> {
    if dims.len() != tail.len() + 1 || dims[1..] != *tail {
        return Err(MeshError::Shape(name));
    }
    let expected = dims
        .iter()
        .try_fold(1_usize, |n, d| n.checked_mul(*d))
        .ok_or(MeshError::Shape(name))?;
    if expected != len {
        return Err(MeshError::Length {
            array: name,
            expected,
            got: len,
        });
    }
    Ok(dims[0])
}

pub(crate) fn narrow(values: &[f64]) -> Result<Vec<f32>, MeshError> {
    values
        .iter()
        .enumerate()
        .map(|(offset, &value)| {
            if !value.is_finite() {
                return Err(MeshError::NonFinite(offset));
            }
            let result = value as f32;
            if value.abs() > f64::from(f32::MAX) || (value != 0.0 && result == 0.0) {
                return Err(MeshError::Narrowing(offset));
            }
            Ok(result)
        })
        .collect()
}

pub(crate) fn contents(
    vertices: usize,
    positions: &[f32],
    indices: &[u32],
) -> Result<(), MeshError> {
    u32::try_from(vertices).map_err(|_| MeshError::VertexCount)?;
    if let Some(offset) = positions.iter().position(|p| !p.is_finite()) {
        return Err(MeshError::NonFinite(offset));
    }
    if let Some((offset, &index)) = indices
        .iter()
        .enumerate()
        .find(|(_, i)| **i as usize >= vertices)
    {
        return Err(MeshError::Index {
            offset,
            index: i64::from(index),
            vertices,
        });
    }
    Ok(())
}

pub(crate) fn one_based(vertices: usize, indices: &[i64]) -> Result<Vec<u32>, MeshError> {
    indices
        .iter()
        .enumerate()
        .map(|(offset, &index)| {
            if index <= 0 || index as u64 > vertices as u64 {
                return Err(MeshError::Index {
                    offset,
                    index,
                    vertices,
                });
            }
            Ok((index - 1) as u32)
        })
        .collect()
}
