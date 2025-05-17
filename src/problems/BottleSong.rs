// https://exercism.org/tracks/rust/exercises/bottle-song

// topics #[string, const]

const NUMBERS: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five",
    "Six", "Seven", "Eight", "Nine", "Ten"
];
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|idx| {
            let current = start_bottles - idx;
            let next = current - 1;

            format_verse(current, next)
        }).collect::<Vec<String>>()
        .join("\n")
}

fn format_verse(current: u32, next: u32) -> String {
    let current_word = get_number_word(current);
    let current_bottles = get_bottles(current);

    let next_word = get_number_word(next).to_lowercase();
    let next_bottles = get_bottles(next);

    format!(
        "{current_word} green {current_bottles} hanging on the wall,\n\
         {current_word} green {current_bottles} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {next_word} green {next_bottles} hanging on the wall.\n",
    )
}

fn get_number_word<'a>(num: u32) -> &'a str {
    if num > NUMBERS.len() as u32 {
        panic!("number ({num}) out of range")
    }

    NUMBERS[num as usize]
}

fn get_bottles<'a>(num: u32) -> &'a str {
    if num == 1 { "bottle" } else { "bottles" }
}
