// https://exercism.org/tracks/rust/exercises/nth-prime

// topics #[number, math]

pub fn nth(n: u32) -> u32 {
    let mut cache: Vec<u32> = vec![];

    (2..).filter(|&candidate| {
        if !cache.iter().any(|primary_number| candidate % primary_number == 0) {
            cache.push(candidate);
            true
        } else { false }
    })
        .nth(n as usize)
        .unwrap()
}


pub fn nth(n: u32) -> u32 {
    (2..).filter(|x| is_primitive(*x))
        .nth(n as usize)
        .unwrap()
}

fn is_primitive(num: u32) -> bool {
    !(2..=num.isqrt()).any(|x| num % x == 0)
}

