use xetal_world_host::{Limits, MeshStore};
use xetal_world_surface::Mesh;

fn triangle() -> Mesh {
    Mesh::from_arrays(&[3, 3], vec![0.0; 9], &[1, 3], vec![0, 1, 2]).unwrap()
}

#[test]
fn disposal_and_reset_never_alias_new_resources() {
    let mut store = MeshStore::new(Limits {
        max_meshes: 2,
        max_bytes: 96,
    });
    let first = store.insert(triangle()).unwrap();
    store.dispose(first).unwrap();
    let second = store.insert(triangle()).unwrap();
    assert_ne!(first, second);
    assert!(store.get(first).is_err());
    assert!(store.dispose(first).is_err());
    store.reset();
    let third = store.insert(triangle()).unwrap();
    assert_ne!(second, third);
    assert!(store.get(second).is_err());
    assert_eq!(store.stats().meshes, 1);
    assert_eq!(store.stats().bytes, 48);
}

#[test]
fn handles_cannot_reach_resources_in_another_store() {
    let limits = Limits {
        max_meshes: 1,
        max_bytes: 48,
    };
    let mut first = MeshStore::new(limits);
    let mut second = MeshStore::new(limits);
    let a = first.insert(triangle()).unwrap();
    let b = second.insert(triangle()).unwrap();
    assert_ne!(a, b);
    assert!(second.get(a).is_err());
    assert!(first.get(b).is_err());
}

#[test]
fn failed_budget_checks_leave_existing_resources_intact() {
    let mut store = MeshStore::new(Limits {
        max_meshes: 2,
        max_bytes: 48,
    });
    let first = store.insert(triangle()).unwrap();
    assert!(store.insert(triangle()).is_err());
    assert_eq!(store.get(first).unwrap().byte_len(), 48);
    assert_eq!(store.stats().bytes, 48);
    store.dispose(first).unwrap();
    assert_eq!(store.stats().bytes, 0);
    assert!(store.insert(triangle()).is_ok());
}

#[test]
fn empty_meshes_still_obey_the_resource_count_limit() {
    let mut store = MeshStore::new(Limits {
        max_meshes: 1,
        max_bytes: 0,
    });
    let empty = || Mesh::from_arrays(&[0, 3], vec![], &[0, 3], vec![]).unwrap();
    assert!(store.insert(empty()).is_ok());
    assert!(store.insert(empty()).is_err());
    store.reset();
    assert_eq!(store.stats().meshes, 0);
    assert!(store.insert(empty()).is_ok());
}
