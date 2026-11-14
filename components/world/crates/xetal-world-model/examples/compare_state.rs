//! Repeatable native evaluator comparison; excludes compilation and startup.
use std::time::Instant;
use xetal_world_model::{Demo, Progress, SOURCE};

const BEFORE: &str = include_str!("../tests/fixtures/snowfield-mutable.xtl");

fn settle(demo: &mut Demo, budget: usize) -> usize {
    let mut slices = 0;
    loop {
        slices += 1;
        if demo.step(budget).unwrap() != Progress::Running {
            return slices;
        }
        assert!(slices < 2_000_000);
    }
}

fn trace() -> Vec<&'static str> {
    if std::env::args().nth(1).as_deref() == Some("ticks") {
        return vec!["tick 0.08"; 64];
    }
    (0..64)
        .map(|i| match i % 16 {
            0 => "key Home",
            7 => "down 0 0",
            15 => "choose 1 0",
            _ => "tick 0.08",
        })
        .collect()
}

fn sample(demo: &mut Demo, events: &[&str]) -> f64 {
    demo.event("key Home").unwrap();
    settle(demo, 8_000);
    let start = Instant::now();
    for event in events {
        demo.event(event).unwrap();
        settle(demo, 8_000);
        std::hint::black_box(demo.mesh().unwrap());
    }
    start.elapsed().as_secs_f64() * 1_000_000.0 / events.len() as f64
}

fn transitions(source: &str, events: &[&str]) -> usize {
    let mut demo = Demo::from_source(source).unwrap();
    settle(&mut demo, 8_000);
    events
        .iter()
        .map(|event| {
            demo.event(event).unwrap();
            settle(&mut demo, 1)
        })
        .sum()
}

fn main() {
    let events = trace();
    let mut before = Demo::from_source(BEFORE).unwrap();
    let mut after = Demo::new().unwrap();
    settle(&mut before, 8_000);
    settle(&mut after, 8_000);
    let mut old = Vec::new();
    let mut new = Vec::new();
    for round in 0..10 {
        if round % 2 == 0 {
            old.push(sample(&mut before, &events));
            new.push(sample(&mut after, &events));
        } else {
            new.push(sample(&mut after, &events));
            old.push(sample(&mut before, &events));
        }
    }
    println!(
        "{{\"events_per_sample\":{},\"before_us_per_event\":{old:?},\"after_us_per_event\":{new:?},\"before_transitions\":{},\"after_transitions\":{}}}",
        events.len(),
        transitions(BEFORE, &events),
        transitions(SOURCE, &events)
    );
}
