// https://exercism.org/tracks/rust/exercises/isogram

pub fn check(candidate: &str) -> bool {
    candidate.chars().fold((0, true),|(mut set, valid), c| {
        if c.is_alphabetic() {
            let code = 1 << (c.to_ascii_lowercase() as u8 - b'a');

            if set & code != 0 {
                return (0, false)
            }

            set |= code;
        }

        (set, valid)
    }).1
}
