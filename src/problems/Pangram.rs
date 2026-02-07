// https://exercism.org/tracks/rust/exercises/pangram

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|it| it.is_alphabetic())
        .fold(0, |mut state: u32, c| {
            state |= 1 << (c as u8 - b'a');
            state
        }) == 2_u32.pow(26) - 1
}


pub fn is_pangram_deep_seek(sentence: &str) -> bool {
    let mut state: u32 = 0;
    for c in sentence.chars().filter(|c| c.is_alphabetic()) {
        if let Some(bit) = (c.to_ascii_lowercase() as u8).checked_sub(b'a') {
            if bit < 26 {
                state |= 1 << bit;
                if state.count_ones() == 26 {
                    return true; // Ранний выход
                }
            }
        }
    }
    state.count_ones() == 26
}