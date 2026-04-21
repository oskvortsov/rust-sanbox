// https://exercism.org/tracks/rust/exercises/say

const ONES: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [&str; 7] = [
    "hundred", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion",
];

fn below_thousand(mut number: u64) -> String {
    let mut parts: Vec<String> = vec![];

    if number >= 100 {
        parts.push(format!("{} {}", ONES[(number / 100) as usize], SCALES[0]));
        number %= 100;
    }

    if number >= 20 {
        let above = number / 10;
        let below = number % 10;

        if below > 0 {
            parts.push(format!("{}-{}", TENS[above as usize], ONES[below as usize]))
        } else {
            parts.push(TENS[above as usize].to_string())
        }
    } else if number > 0 {
        parts.push(ONES[number as usize].to_string())
    }

    parts.join(" ")
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return ONES[0].to_string()
    }

    let mut parts: Vec<String> = vec![];
    let mut scale_idx = 0;

    while n > 0 {
        let rest = n % 1000;

        if rest > 0 {
            let mut rest_word = below_thousand(rest);

            if scale_idx > 0 {
                rest_word = format!("{} {}", rest_word, SCALES[scale_idx].to_string())
            }

            parts.push(rest_word);
        }

        n /= 1000;
        scale_idx += 1;
    };

    parts.reverse();
    parts.join(" ")
}

use std::iter::successors;

pub fn encode2(num: u64) -> String {
    match num {
        0..=19 => ONES[num as usize].to_string(),
        20..=99 => {
            let upper = (num / 10) as usize;
            match num % 10 {
                0 => TENS[upper].to_string(),
                lower => format!("{}-{}", TENS[upper], encode(lower)),
            }
        }
        100..=999 => format_num(num, 100, "hundred"),
        _ => {
            let (div, order) =
                successors(Some(1u64), |v| v.checked_mul(1000))
                    .zip(SCALES.iter())
                    .find(|&(e, _)| e > num / 1000)
                    .unwrap();
            format_num(num, div, order)
        }
    }
}
fn format_num(num: u64, div: u64, order: &str) -> String {
    match (num / div, num % div) {
        (upper, 0) => format!("{} {}", encode(upper), order),
        (upper, lower) => {
            format!("{} {} {}", encode(upper), order, encode(lower))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_one() {
//         println!("{}", encode(1_123_234_777));
//
//     }
// }
