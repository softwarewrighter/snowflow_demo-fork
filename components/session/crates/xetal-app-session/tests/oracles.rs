//! Compare new composition with the retained standalone programs, tick for tick.
use xetal_app_session::{Pose, Session};
use xetal_world_model::{Demo, Progress};
fn settle(demo: &mut Demo) {
    for _ in 0..4096 {
        if demo.step(8000).unwrap() == Progress::Waiting {
            return;
        }
    }
    panic!("reference did not settle");
}
fn event(demo: &mut Demo, event: &str) {
    demo.event(event).unwrap();
    settle(demo);
}
fn baseline(name: &str) -> Demo {
    let text = match name {
        "ice" => include_str!("../../../../world/demos/crystallise.xtl"),
        "slush" => include_str!("../../../../world/demos/sweep.xtl"),
        "water" => include_str!("../../../../world/demos/ribbon.xtl"),
        _ => include_str!("../../../../world/demos/bloom.xtl"),
    };
    let mut demo = Demo::from_source(text).unwrap();
    settle(&mut demo);
    demo
}
#[test]
fn crystallise_and_sweep_geometry_match_standalone_replays() {
    for (key, material) in [("4", "ice"), ("1", "slush")] {
        let mut app = Session::new().unwrap();
        let mut old = baseline(material);
        event(&mut old, "move 0 8");
        event(&mut old, "down 0 0");
        event(&mut old, "click 0 0");
        app.input(key, true).unwrap();
        for _ in 0..30 {
            app.tick(0.1).unwrap();
            event(&mut old, "tick 0.1");
            let frame = app.read().unwrap();
            assert_eq!(
                frame
                    .resources
                    .iter()
                    .find(|r| r.material == material)
                    .unwrap()
                    .mesh
                    .positions(),
                old.mesh().unwrap().positions()
            );
        }
    }
}
#[test]
fn bloom_geometry_and_particles_match_after_generic_query_response() {
    let mut app = Session::new().unwrap();
    let mut old = baseline("bloom");
    app.input("3", true).unwrap();
    let query = app.read().unwrap().requests;
    assert_eq!(&query[..2], &[1.0, 1.0]);
    app.respond(1, Some([2.0, 0.5, -4.0])).unwrap();
    for text in ["move 2 -4", "down 0.5 0", "click 0 0"] {
        event(&mut old, text);
    }
    for _ in 0..60 {
        app.tick(0.1).unwrap();
        event(&mut old, "tick 0.1");
        let frame = app.read().unwrap();
        assert_eq!(
            frame
                .resources
                .iter()
                .find(|r| r.material == "slush")
                .unwrap()
                .mesh
                .positions(),
            old.mesh().unwrap().positions()
        );
        let (_, particles) = old.array(2).unwrap();
        assert_eq!(
            frame.particles,
            particles.iter().map(|n| *n as f32).collect::<Vec<_>>()
        );
    }
}
#[test]
fn ribbon_held_and_released_geometry_match_standalone_replay() {
    let mut app = Session::new().unwrap();
    let mut old = baseline("water");
    let pose = Pose {
        position: [0.0, 4.0, 8.0],
        aim: [0.0, -0.12],
        grounded: true,
        impact: 0.0,
    };
    app.pose(pose).unwrap();
    app.input("2", true).unwrap();
    for text in ["move 0 8", "down 4 0", "up 0 -0.12", "click 1 0"] {
        event(&mut old, text);
    }
    for tick in 0..50 {
        if tick == 10 {
            app.input("2", false).unwrap();
            event(&mut old, "click 0 0");
        }
        app.tick(0.05).unwrap();
        event(&mut old, "tick 0.05");
        let frame = app.read().unwrap();
        assert_eq!(
            frame
                .resources
                .iter()
                .find(|r| r.material == "water")
                .unwrap()
                .mesh
                .positions(),
            old.mesh().unwrap().positions()
        );
    }
}
#[test]
fn footsteps_and_wind_match_the_original_player_program() {
    let mut app = Session::new().unwrap();
    let mut old = Demo::new().unwrap();
    settle(&mut old);
    for step in 1..=40 {
        let z = 8.0 - step as f32 * 0.35;
        app.tick(0.1).unwrap();
        event(&mut old, "tick 0.1");
        app.pose(Pose {
            position: [0.0, 2.0, z],
            aim: [0.0, -0.12],
            grounded: true,
            impact: 0.0,
        })
        .unwrap();
        event(&mut old, &format!("move 0 {z}"));
        event(&mut old, "down 0 0");
        let expected = old.commands().unwrap();
        let actual = app.read().unwrap();
        assert_eq!(actual.stats["steps"], expected.steps as f64);
        assert_eq!(
            actual.prints,
            expected.prints.into_iter().flatten().collect::<Vec<_>>()
        );
        if expected.sound[0] != 0.0 {
            assert_eq!(actual.audio, expected.sound);
        }
    }
}
