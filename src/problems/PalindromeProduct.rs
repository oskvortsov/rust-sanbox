use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64) -> Self {
        Self {
            value,
            factors: HashSet::new(),
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }

    pub fn add_factor(&mut self, factor: (u64, u64)) {
        let sorted_factor = if factor.0 > factor.1 {
            (factor.1, factor.0)
        } else {
            factor
        };

        self.factors.insert(sorted_factor);
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut smallest: Palindrome = Palindrome::new(u64::MAX);
    let mut greatest: Palindrome = Palindrome::new(u64::MIN);
    let mut is_palindrome_found = false;

    (min..=max).for_each(|first| {
        (min..=max).for_each(|second| {
            let product = first * second;

            if is_palindrome(product) {
                is_palindrome_found = true;

                if smallest.value >= product {
                    if smallest.value > product {
                        smallest = Palindrome::new(product);
                    }
                    smallest.add_factor((first, second));
                }

                if greatest.value <= product {
                    if greatest.value < product {
                        greatest = Palindrome::new(product);
                    }
                    greatest.add_factor((first, second));
                }
            }
        })
    });

    if !is_palindrome_found {
        return None;
    }

    Some((smallest, greatest))
}

fn is_palindrome(original: u64) -> bool {
    let mut current = original;
    let mut reversed = 0;

    while current > 0 {
        reversed = reversed * 10 + current % 10;
        current /= 10;
    }

    original == reversed
}
