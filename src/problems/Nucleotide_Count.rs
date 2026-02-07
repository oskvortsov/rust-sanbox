// https://exercism.org/tracks/rust/exercises/nucleotide-count

use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut nucleotides = nucleotide_counts(dna)?;
    nucleotides.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = ['A', 'C', 'G', 'T'].iter().map(|&n| (n, 0)).collect();

    for c in dna.chars() {
        counts.get_mut(&c).map(|count| *count += 1).ok_or(c)?
    }

    Ok(counts)
}


// // My solution
// use std::collections::HashMap;
//
// const NUCLEOTIDE_MASK: u32 = 1 // A char
//     | (1 << ('C' as u32 - 'A' as u32))
//     | (1 << ('G' as u32 - 'A' as u32))
//     | (1 << ('T' as u32 - 'A' as u32));
//
// pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
//     let nucleotide_code: u32 = 1 << (nucleotide as u32 - 'A' as u32);
//
//     if NUCLEOTIDE_MASK & nucleotide_code == 0 {
//         return Err(nucleotide);
//     }
//
//     let mut count = 0;
//
//     for c in dna.chars() {
//         let code = 1 << (c as u32 - 'A' as u32);
//
//         if NUCLEOTIDE_MASK & code == 0 {
//             return Err(c);
//         }
//
//         if c == nucleotide {
//             count += 1;
//         }
//     }
//
//     Ok(count)
// }
//
// pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
//     ['A', 'C', 'G', 'T']
//         .iter()
//         .map(|&n| count(n, dna).map(|c| (n, c)))
//         .collect::<Result<HashMap<_, _>, _>>()
// }
