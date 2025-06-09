// https://exercism.org/tracks/rust/exercises/allergies

// topics #[Enum, bit manipulation, iterator]

use self::Allergen::*;

pub struct Allergies(u8);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGENS: [Allergen; 8] =
    [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

impl Allergen {
    fn bit(&self) -> u8 {
        match *self {
            Eggs => 1 << 0,
            Peanuts => 1 << 1,
            Shellfish => 1 << 2,
            Strawberries => 1 << 3,
            Tomatoes => 1 << 4,
            Chocolate => 1 << 5,
            Pollen => 1 << 6,
            Cats => 1 << 7,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score as u8)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & allergen.bit() != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS.iter()
            .filter(|x| self.is_allergic_to(x))
            .cloned().collect()
    }
}
