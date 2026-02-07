// https://exercism.org/tracks/rust/exercises/run-length-encoding

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    let mut count = 0;

    while let Some(cur) = chars.next() {
        count += 1;

        if chars.peek() != Some(&cur) {
            if count > 1 {
                result.push_str(&count.to_string());
            }

            result.push(cur);
            count = 0;
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut numeric = String::new();
    let mut result = String::new();

    for cur in source.chars() {
        if cur.is_numeric() {
            numeric.push(cur);
        } else {
            result.push_str(
                &cur.to_string()
                    .repeat(numeric.parse::<usize>().unwrap_or(1)),
            );
            numeric.clear();
        }
    }

    result
}
