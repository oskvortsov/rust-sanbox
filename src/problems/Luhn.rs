// https://exercism.org/tracks/rust/exercises/luhn

// topics #[chars, map, fold, try_fold, tuple]

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code
        .chars()
        .rev()
        .filter(|x| !x.is_whitespace())
        .try_fold((0 , 0), |(sum, count), x| {
            x.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2} else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}


/// Check a Luhn checksum.
pub fn is_valid_1(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false
    }

    let char_arr: Vec<u8> = code.as_bytes()
        .iter()
        .filter(|&x| !x.is_ascii_whitespace())
        .copied()
        .collect();

    let parity = char_arr.len() % 2;
    let mut sum: usize = 0;

    for i in 0..char_arr.len() {
        let char = char_arr[i];

        if !char.is_ascii_digit() {
            return false
        }

        let mut digit = (char - '0' as u8) as usize;

        if i % 2 == parity {
            digit *= 2;
            if digit > 9 { digit -= 9 }
        }

        sum += digit;
    }

    let calculated_check_digit = (10 - (sum % 10)) % 10;

    calculated_check_digit % 10 == 0
}