//! Native payload microbenchmark; does not measure an evaluator, FFI or GPU.
use std::hint::black_box;
use std::time::Instant;
use xetal_world_surface::{Mesh, from_xetal};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const TRIANGLES: usize = 4096;
const ROUNDS: usize = 32;

fn owned(points: &[f64], indices: &[i64]) -> Result<f64> {
    let prepare = || {
        let points = points.iter().map(|v| *v as f32).collect::<Vec<_>>();
        let indices = indices.iter().map(|i| (*i - 1) as u32).collect::<Vec<_>>();
        (points, indices)
    };
    let batches = (0..ROUNDS).map(|_| prepare()).collect::<Vec<_>>();
    let start = Instant::now();
    for (points, indices) in batches {
        let mesh = Mesh::from_arrays(&[TRIANGLES * 3, 3], points, &[TRIANGLES, 3], indices)?;
        black_box(mesh);
    }
    Ok(start.elapsed().as_secs_f64() * 1000.0)
}

fn checked(points: &[f64], indices: &[i64]) -> Result<f64> {
    let start = Instant::now();
    for _ in 0..ROUNDS {
        black_box(from_xetal(
            &[TRIANGLES * 3, 3],
            points,
            &[TRIANGLES, 3],
            indices,
        )?);
    }
    Ok(start.elapsed().as_secs_f64() * 1000.0)
}

fn text(points: &[f64], indices: &[i64]) -> Result<f64> {
    let start = Instant::now();
    for _ in 0..ROUNDS {
        let points = roundtrip(points)?;
        let indices = roundtrip(indices)?;
        black_box(from_xetal(
            &[TRIANGLES * 3, 3],
            &points,
            &[TRIANGLES, 3],
            &indices,
        )?);
    }
    Ok(start.elapsed().as_secs_f64() * 1000.0)
}

fn roundtrip<T>(values: &[T]) -> Result<Vec<T>>
where
    T: std::fmt::Display + std::str::FromStr,
    T::Err: std::error::Error + 'static,
{
    Ok(values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<Vec<_>, _>>()?)
}

fn main() -> Result<()> {
    let points = (0..TRIANGLES * 9)
        .map(|n| (n % 101) as f64 / 100.0)
        .collect::<Vec<_>>();
    let indices = (1..=TRIANGLES * 3).map(|n| n as i64).collect::<Vec<_>>();
    println!(
        "{{\"triangles\":{TRIANGLES},\"rounds\":{ROUNDS},\"owned_f32_ms\":{},\"checked_f64_ms\":{},\"text_roundtrip_ms\":{}}}",
        owned(&points, &indices)?,
        checked(&points, &indices)?,
        text(&points, &indices)?
    );
    Ok(())
}
