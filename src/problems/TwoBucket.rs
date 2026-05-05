use std::collections::HashSet;
use crate::Bucket::{One, Two};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

struct BucketStat {
    filled: u8,
    capacity: u8
}

impl BucketStat {
    fn new(capacity: u8) -> Self {
        BucketStat {
            filled: 0,
            capacity
        }
    }

    fn filling(&mut self, litters: u8) -> u8 {
        self.filled += litters;

        if self.filled > self.capacity {
            let remains = self.filled - self.capacity;
            self.filled = self.capacity;

            return remains
        }

        0
    }

    fn filled(&mut self) {
        self.filled = self.capacity;
    }

    fn pouring(&mut self) {
        self.filled = 0;
    }

    fn is_full(&self) -> bool {
        self.filled == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.filled == 0
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1.max(capacity_2) {
        return None
    }

    let mut moves: u8 = 1;
    let mut bucket_1 = BucketStat::new(capacity_1);
    let mut bucket_2 = BucketStat::new(capacity_2);
    let mut steps: HashSet<(u8, u8)> = HashSet::new();


    if *start_bucket == One {
        bucket_1.filled()
    } else {
        bucket_2.filled()
    };

    let (pouring, refillable) = if *start_bucket == One {
        (&mut bucket_1, &mut bucket_2)
    } else {
        (&mut bucket_2, &mut bucket_1)
    };

    if refillable.capacity == goal {
        refillable.filled();
        moves += 1;
    }

    while pouring.filled != goal && refillable.filled != goal {

        if steps.contains(&(pouring.filled, refillable.filled)) {
            return None
        }

        steps.insert((pouring.filled, refillable.filled));

        pouring.filled = refillable.filling(pouring.filled);
        moves += 1;

        if pouring.filled == goal || refillable.filled == goal {
            break
        }

        if pouring.is_empty() {
            pouring.filled();
            moves += 1;
        } else if refillable.is_full() {
            refillable.pouring();
            moves += 1;
        }
    }

    Some(BucketStats {
        moves,
        goal_bucket: if bucket_1.filled == goal { One } else { Two },
        other_bucket: if bucket_1.filled == goal { bucket_2.filled } else { bucket_1.filled }
    })
}

pub fn solve2(
    cap_1: u8,
    cap_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let (cap, mut state, backs, mut moves) = match start_bucket {
        Bucket::One => ([cap_1, cap_2], [cap_1, 0], [Bucket::One, Bucket::Two], 1),
        Bucket::Two => ([cap_2, cap_1], [cap_2, 0], [Bucket::Two, Bucket::One], 1),
    };
    while state.iter().all(|v| *v != goal) {
        match state {
            [_, _] if cap[1] == goal => state[1] = goal,
            [0, _] => state[0] = cap[0],
            [c1, c2] if c1 < cap[0] && c2 == cap[1] => state[1] = 0,
            [c1, c2] if c1 <= cap[0] && c2 < cap[1] => {
                state[0] = c1 - c1.min(cap[1] - c2);
                state[1] = c2 + c1.min(cap[1] - c2);
            }
            _ => return None,
        }
        moves += 1;
    }
    let (goal_bucket, other_bucket) = match state {
        [_, c] if c == goal => (backs[1], state[0]),
        _ => (backs[0], state[1]),
    };
    Some(BucketStats { goal_bucket, other_bucket, moves })
}