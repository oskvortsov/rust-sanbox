// https://exercism.org/tracks/rust/exercises/proverb

// topics #[string, template, iterator]

use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(first_word) => list.windows(2)
            .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1]))
            .chain(once(format!("And all for the want of a {}.", first_word)))
            .collect()
    }
}
