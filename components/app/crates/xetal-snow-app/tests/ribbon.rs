use xetal_snow_app::Scene;
fn ready() -> Scene {
    let mut scene = Scene::new().unwrap();
    for _ in 0..1000 {
        scene.update(0.0, true).unwrap();
        if scene.world.resident() == 25 {
            return scene;
        }
    }
    panic!("scene did not load");
}
#[test]
fn held_ribbon_tracks_pose_pauses_and_clears_on_reset() {
    let mut scene = ready();
    scene.input("2", true).unwrap();
    for _ in 0..15 {
        scene.update(0.05, false).unwrap();
    }
    assert_eq!(scene.frame.stats["ribbon-mode"] as i32, 1);
    assert!(
        !scene
            .frame
            .resources
            .iter()
            .find(|r| r.material == "water")
            .unwrap()
            .mesh
            .positions()
            .is_empty()
    );
    let old = scene
        .frame
        .resources
        .iter()
        .find(|r| r.material == "water")
        .unwrap()
        .mesh
        .positions()
        .to_vec();
    scene.update(0.1, true).unwrap();
    assert!(
        old == scene
            .frame
            .resources
            .iter()
            .find(|r| r.material == "water")
            .unwrap()
            .mesh
            .positions()
    );
    scene.input("2", false).unwrap();
    for _ in 0..80 {
        scene.update(0.05, false).unwrap();
    }
    assert_eq!(scene.frame.stats["ribbon-mode"] as i32, 0);
    assert!(
        scene
            .frame
            .resources
            .iter()
            .find(|r| r.material == "water")
            .unwrap()
            .mesh
            .positions()
            .is_empty()
    );
    scene.input("2", true).unwrap();
    scene.update(0.05, false).unwrap();
    scene.input("r", true).unwrap();
    assert!(
        scene
            .update(0.0, true)
            .unwrap()
            .resources
            .iter()
            .find(|r| r.material == "water")
            .unwrap()
            .mesh
            .positions()
            .is_empty()
    );
}
