// https://exercism.org/tracks/rust/exercises/difference-of-squares

// topics #[number, math]

// articles
// https://dev.mccme.ru/~merzon/pscache/bernoulli-elem-pre.html

pub fn square_of_sum(n: u32) -> u32 {
    // (0..n + 1).fold(0, |acc, x| { acc + x }).pow(2)
    // (0..=n).sum::<u32>().pow(2)
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // (0..n + 1).fold(0, |acc ,x| { acc + x.pow(2) })
    // (0..=n).map(|x| x.pow(2)).sum()
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    sum_of_squares(n).abs_diff(square_of_sum(n))
}