// https://exercism.org/tracks/rust/exercises/series

pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|window| window.iter().collect::<String>())
        .collect()
}
