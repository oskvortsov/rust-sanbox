// https://exercism.org/tracks/rust/exercises


/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars().fold((0, 0, true), |(count, sum, valid), c| {
        match c {
            '0'..='9' => (
                count + 1,
                // mod - Дистрибутивная операция (a + b) mod m = [(a mod m) + (b mod m)] mod m
                (sum + (10 - count) * c.to_digit(10).unwrap()) % 11,
                valid,
            ),
            'X' => (count + 1, (sum + 10) % 11, valid && count == 9),
            '-' => (count, sum, valid),
            _ => (0, 0, false)
        }
    }) == (10, 0, true)

    // let mut digits: Vec<u32> = vec![];
    //
    // for c in isbn.chars() {
    //     if let Some(digit) = c.to_digit(10) {
    //         digits.push(digit);
    //         continue
    //     }
    //
    //     if c == 'X' {
    //
    //         if digits.len() < 9 {
    //             return false;
    //         }
    //
    //         digits.push(10);
    //         continue
    //     }
    //
    //     if c == '-' {
    //         continue
    //     }
    //
    //     return false
    // }
    //
    // if digits.len() != 10 {
    //     return false
    // }
    //
    // digits.iter().enumerate()
    //     .map(|(idx, &val)| val * (10 - idx as u32))
    //     .sum::<u32>()
    // % 11 == 0
}
