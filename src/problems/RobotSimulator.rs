// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use crate::Direction::{East, North, South, West};

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_idx(idx: u8) -> Self {
        match idx {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => panic!()
        }
    }

    fn to_idx(self) -> u8 {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }
}

pub struct Robot {
    state: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            state: (x, y),
            dir: d
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            state: self.state,
            dir: Direction::from_idx((self.dir.to_idx() + 1) % 4)
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            state: self.state,
            dir: Direction::from_idx((self.dir.to_idx() + 3) % 4)
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let diff = self.diff();

        Self {
            state: (self.state.0 + diff.0, self.state.1 + diff.1),
            dir: self.dir
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, code| {
            match code {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => panic!()
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.state
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }

    fn diff(&self) -> (i32, i32) {
        match self.dir {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }
}
