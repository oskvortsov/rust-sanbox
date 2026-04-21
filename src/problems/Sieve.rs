// https://exercism.org/tracks/rust/exercises/sieve
use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut not_primitive: HashSet<u64> = HashSet::new();

    for p in 2..=upper_bound {
        if not_primitive.contains(&p) { continue }

        result.push(p);

        let mut index = 1;
        let mut cur = p * p;

        while cur <= upper_bound {
            not_primitive.insert(cur);
            cur = index * p;
            index += 1;
        }
    }

    result
}
