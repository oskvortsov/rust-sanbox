// https://exercism.org/tracks/rust/exercises/etl

use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input
        .iter()
        .flat_map(|(&point, list)| {
            list.iter().map(move |&c| (c.to_ascii_lowercase(), point))
        })
        .collect()
}
