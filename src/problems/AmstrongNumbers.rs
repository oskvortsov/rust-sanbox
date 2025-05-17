// https://exercism.org/tracks/rust/exercises/armstrong-numbers

// topics #[fold, map]

pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len() as u32;

    num == num.to_string().chars()
        .fold(0, |sum, x| x.to_digit(10).unwrap().pow(len) + sum)
}

pub fn is_armstrong_number_1(num: u32) -> bool {
    let len = num.to_string().len() as u32;

    num.to_string()
        .chars()
        .try_fold(0, |sum, x| x.to_digit(10).map(|num| sum + num.pow(len)))
        .map_or(false, |sum| sum == num)
}

