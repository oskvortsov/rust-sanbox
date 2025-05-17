// https://exercism.org/tracks/rust/exercises/acronym

// topics #[chars, string, map, peekable]

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            let mut result = vec![];
            let mut chars = word.chars().peekable();

            while let Some(c) = chars.next() {
                if !c.is_ascii_alphabetic() {
                    continue;
                }

                // Take first letter of each word
                if result.is_empty() {
                    result.push(c.to_ascii_uppercase());
                    continue;
                }

                // Handle camelCase and consecutive capitals
                if !c.is_ascii_uppercase() {
                    if let Some(&next) = chars.peek() {
                        if next.is_ascii_uppercase() {
                            result.push(next.to_ascii_uppercase());
                            chars.next(); // Skip the peeked char
                        }
                    }
                }
            }

            result
        })
        .collect()
}

// test case 'all_caps_word' shows that a 'word' for the purpose of this test is either:
// *a lower case word.
// *a word starting with an uppercase letter.
// *a word with all-caps.
//
// Terminal apostrophes are not considered extra words.
//
// The application is english-based, so we can work with ascii.
// enum AbbreviateState {
//     AllCapsWord,
//     CheckCaps,
//     LowerCaseWord,
//     Whitespace,
// }
// pub fn abbreviate_state(phrase: &str) -> String {
//     use AbbreviateState::*;
//     let mut result = String::new();
//     let mut state: AbbreviateState = Whitespace;
//     for i in phrase.chars() {
//         match i {
//             'A'..='Z' => match state {
//                 Whitespace => {
//                     result.push(i);
//                     state = CheckCaps;
//                 }
//                 CheckCaps => {
//                     state = AllCapsWord;
//                 }
//                 LowerCaseWord => {
//                     result.push(i);
//                     state = CheckCaps;
//                 }
//                 AllCapsWord => {}
//             },
//             'a'..='z' => match state {
//                 Whitespace => {
//                     result.push(i.to_ascii_uppercase());
//                     state = LowerCaseWord;
//                 }
//                 CheckCaps => {
//                     state = LowerCaseWord;
//                 }
//                 AllCapsWord | LowerCaseWord => {}
//             },
//             '\'' => {}
//             _ => {
//                 state = Whitespace;
//             }
//         }
//     }
//     result
// }
//
// pub fn abbreviate_community(phrase: &str) -> String {
//     phrase
//         .split(|c: char| c.is_ascii_whitespace() || c  == '-' || c == '_')
//         .flat_map(|word| {
//             word.chars().take(1)
//                 .chain(
//                     word.chars()
//                         .skip_while(|c| c.is_uppercase())
//                         .filter(|c| c.is_uppercase())
//                 )
//         })
//         .collect::<String>()
//         .to_uppercase()
// }

// fn filter_abbreviate(word: &str) -> String {
//     let null_char = '\0' as u8;
//     let mut prev = null_char;
//
//     word.bytes()
//         .filter(|x| x.is_ascii_alphabetic())
//         .filter(|x| {
//             let mut is_abbreviate_symbol = false;
//
//             if prev == null_char || x.is_ascii_uppercase() && !prev.is_ascii_uppercase() {
//                 is_abbreviate_symbol = true;
//             }
//
//             prev = *x;
//             is_abbreviate_symbol
//         })
//         .map(|x| x.to_ascii_uppercase() as char)
//         .collect()
// }
