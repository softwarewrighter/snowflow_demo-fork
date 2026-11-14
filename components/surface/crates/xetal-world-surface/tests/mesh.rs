use xetal_world_surface::{Mesh, from_scene, from_xetal};

#[test]
fn owned_typed_buffers_are_retained_without_repacking() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let indices = vec![0, 1, 2];
    let pointers = (positions.as_ptr(), indices.as_ptr());
    let mesh = Mesh::from_arrays(&[3, 3], positions, &[1, 3], indices).unwrap();
    assert_eq!(mesh.positions().as_ptr(), pointers.0);
    assert_eq!(mesh.indices().as_ptr(), pointers.1);
    assert_eq!(mesh.byte_len(), 48);
}

#[test]
fn scene_triangle_order_and_winding_are_preserved() {
    let points = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let mesh = from_scene(&[1, 3, 3], &points).unwrap();
    assert_eq!(mesh.positions(), points.map(|v| v as f32));
    assert_eq!(mesh.indices(), [0, 1, 2]);
    assert!(from_scene(&[1, 9], &points).is_err());
}

#[test]
fn xetal_indices_are_checked_before_one_based_conversion() {
    let points = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let mesh = from_xetal(&[3, 3], &points, &[1, 3], &[1, 2, 3]).unwrap();
    assert_eq!(mesh.indices(), [0, 1, 2]);
    for bad in [0, -1, 4, i64::MAX] {
        assert!(from_xetal(&[3, 3], &points, &[1, 3], &[1, 2, bad]).is_err());
    }
}

#[test]
fn malformed_shapes_and_nonfinite_values_fail() {
    for shape in [&[9][..], &[3, 2][..], &[usize::MAX, 3][..]] {
        assert!(Mesh::from_arrays(shape, vec![0.0; 9], &[0, 3], vec![]).is_err());
    }
    assert!(Mesh::from_arrays(&[3, 3], vec![0.0; 8], &[0, 3], vec![]).is_err());
    assert!(Mesh::from_arrays(&[1, 3], vec![f32::NAN; 3], &[0, 3], vec![]).is_err());
    assert!(Mesh::from_arrays(&[1, 3], vec![0.0; 3], &[1, 3], vec![0, 1, 2]).is_err());
    assert!(Mesh::from_arrays(&[1, 3], vec![0.0; 3], &[1, 2], vec![0, 0]).is_err());
}

#[test]
fn precision_policy_rejects_overflow_underflow_and_nonfinite_input() {
    for bad in [f64::MAX, f64::MIN_POSITIVE, f64::NAN, f64::INFINITY] {
        assert!(from_scene(&[1, 3, 3], &[bad; 9]).is_err());
    }
    let mesh = from_scene(&[1, 3, 3], &[0.1; 9]).unwrap();
    assert_eq!(mesh.positions(), [0.1_f32; 9]);
    assert_eq!(from_scene(&[0, 3, 3], &[]).unwrap().byte_len(), 0);
}
