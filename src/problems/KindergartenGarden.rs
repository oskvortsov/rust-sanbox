// https://exercism.org/tracks/rust/exercises/kindergarten-garden

// #topic []
const STUDENTS: [&str; 12] = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_idx = STUDENTS.iter()
        .position(|&s| s == student).unwrap() * 2;

    diagram.lines()
        .flat_map(|line| line[student_idx..student_idx + 2].chars())
        .map(|code| {
            match code {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => ""
            }
        }).collect()
}
