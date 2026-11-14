pub type Key = (i32, i32);

pub fn center(position: [f32; 3]) -> Key {
    (
        (position[0] / 32.0).floor() as i32,
        (position[2] / 32.0).floor() as i32,
    )
}

pub fn ring((cx, cz): Key) -> Vec<Key> {
    let mut keys: Vec<_> = (-2..=2)
        .flat_map(|x| (-2..=2).map(move |z| (cx.saturating_add(x), cz.saturating_add(z))))
        .collect();
    keys.sort_by_key(|&(x, z)| {
        (i64::from(x) - i64::from(cx)).pow(2) + (i64::from(z) - i64::from(cz)).pow(2)
    });
    keys.dedup();
    keys
}
