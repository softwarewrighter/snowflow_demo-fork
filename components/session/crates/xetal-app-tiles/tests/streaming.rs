use xetal_app_session::Session;
use xetal_app_tiles::Tiles;
#[test]
fn one_app_generates_and_revises_its_resident_chunks() {
    let mut session = Session::new().unwrap();
    let mut tiles = Tiles::default();
    for _ in 0..25 {
        tiles.step(&mut session, [0.0, 4.0, 8.0]).unwrap();
    }
    assert_eq!(tiles.resident(), 25);
    assert!(tiles.step(&mut session, [0.0, 4.0, 8.0]).unwrap().is_none());
    session.input("1", true).unwrap();
    session.tick(0.1).unwrap();
    let changed = tiles.step(&mut session, [0.0, 4.0, 8.0]).unwrap().unwrap();
    assert_eq!(changed.indices().len(), 25 * 512 * 3);
    assert!(tiles.edit_revision > 0);
    assert!(session.read().unwrap().dirty.is_empty());
}
#[test]
fn residency_remains_bounded_and_edits_survive_eviction_and_revisit() {
    let mut app = Session::new().unwrap();
    let mut tiles = Tiles::default();
    for _ in 0..25 {
        tiles.step(&mut app, [0.0, 4.0, 8.0]).unwrap();
    }
    app.input("1", true).unwrap();
    for _ in 0..5 {
        app.tick(0.1).unwrap();
    }
    let edited = tiles.step(&mut app, [0.0, 4.0, 8.0]).unwrap().unwrap();
    for _ in 0..25 {
        tiles.step(&mut app, [192.0, 4.0, 8.0]).unwrap();
        assert!(tiles.resident() <= 25);
    }
    let mut returned = None;
    for _ in 0..25 {
        returned = tiles.step(&mut app, [0.0, 4.0, 8.0]).unwrap().or(returned);
    }
    assert_eq!(returned.unwrap().positions(), edited.positions());
}
