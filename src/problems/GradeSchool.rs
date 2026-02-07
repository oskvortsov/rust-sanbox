// https://exercism.org/tracks/rust/exercises/grade-school

use std::collections::{BTreeMap, BinaryHeap, HashSet};

pub struct School {
    grades: BTreeMap<u32, BinaryHeap<String>>,
    students: HashSet<String>
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
            students: HashSet::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.students.contains(student) {
            self.grades.entry(grade).or_insert(BinaryHeap::new()).push(student.to_string());
            self.students.insert(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades.get(&grade).cloned().unwrap_or(BinaryHeap::new()).into_sorted_vec()
    }
}


// use std::cmp::Reverse;
// use std::collections::{BinaryHeap, HashMap, HashSet};
// pub struct School {
//     table: HashMap<u32, BinaryHeap<Reverse<String>>>,
//     students: HashSet<String>
// }
// impl School {
//     pub fn new() -> School {
//         School {
//             table: HashMap::new(),
//             students: HashSet::new()
//         }
//     }
//     pub fn add(&mut self, grade: u32, student: &str) {
//         if self.students.contains(student) {
//             return;
//         }
//         if !self.table.contains_key(&grade) {
//             self.table.insert(grade, BinaryHeap::new());
//         }
//         let grade_item = self.table.get_mut(&grade).unwrap();
//         self.students.insert(student.to_string());
//         grade_item.push(Reverse(student.to_string()));
//     }
//     pub fn grades(&self) -> Vec<u32> {
//         let mut grades: Vec<u32> = self.table.keys().map(|&x| x).collect();
//         grades.sort();
//         grades
//     }
//     // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
//     // internally to lend out. By returning an owned vector of owned `String`s instead,
//     // the internal structure can be completely arbitrary. The tradeoff is that some data
//     // must be copied each time `grade` is called.
//     pub fn grade(&self, grade: u32) -> Vec<String> {
//         if let Some(students) = self.table.get(&grade) {
//             let sorted_students = students.iter().map(|Reverse(s)| s.clone());
//             return sorted_students.collect();
//         }
//         vec![]
//     }
// }
