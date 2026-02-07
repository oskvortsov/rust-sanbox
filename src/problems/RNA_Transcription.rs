// https://exercism.org/tracks/rust/exercises/rna-transcription

const DNA_NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDES: [char; 4] = ['C', 'G', 'A', 'U'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: String,
}

fn validate_sequence(sequence: &str, group: [char; 4]) -> Result<String, usize> {
    match sequence.chars().position(|c| !group.contains(&c)) {
        Some(idx) => Err(idx),
        None => Ok(sequence.to_string()),
    }
}

fn transcribe(nucleotide: char) -> char {
    RNA_NUCLEOTIDES[DNA_NUCLEOTIDES
        .iter()
        .position(|&c| c == nucleotide)
        .unwrap()]
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate_sequence(dna, DNA_NUCLEOTIDES).map(|dna| Self { nucleotides: dna })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            nucleotides: self.nucleotides.chars().map(|c| transcribe(c)).collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate_sequence(rna, RNA_NUCLEOTIDES).map(|rna| Self { nucleotides: rna })
    }
}
