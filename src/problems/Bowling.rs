// https://exercism.org/tracks/rust/exercises/bowling

use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_frame: u8,
    current_throw: u8,
    throws: Vec<u16>,
    pins_left: i16,
    bonus_throw: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_frame: 1,
            current_throw: 0,
            pins_left: 10,
            throws: vec![],
            bonus_throw: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        self.pins_left -= pins as i16;
        self.throws.push(pins);

        if self.pins_left < 0 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.bonus_throw > 0 {
            if self.pins_left == 0 {
                self.pins_left = 10;
            }

            self.bonus_throw -= 1;

            if self.bonus_throw == 0 {
                self.current_frame += 1;
            }

            return Ok(());
        }

        self.current_throw += 1;

        let is_all_pins_down = self.pins_left == 0;
        let is_last_frame = self.current_frame == 10;
        let is_second_throw = self.current_throw == 2;

        if is_all_pins_down {
            if is_last_frame {
                match self.current_throw {
                    1 => self.bonus_throw = 2,
                    2 => self.bonus_throw = 1,
                    _ => (),
                }
            } else {
                self.current_frame += 1;
            }

            self.pins_left = 10;
            self.current_throw = 0;
        } else if is_second_throw {
            self.current_frame += 1;
            self.pins_left = 10;
            self.current_throw = 0;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }

        let mut total_score = 0;
        let mut idx = 0;
        let mut frame_idx = 1;

        while idx < self.throws.len() {
            let score = self.throws[idx];

            // last frame
            if frame_idx == 10 {
                total_score += score;
                idx += 1;
                continue
            }

            // strike
            if score == 10 {
                total_score += score
                    + self.throws.get(idx + 1).unwrap_or(&0)
                    + self.throws.get(idx + 2).unwrap_or(&0);
                idx += 1;
            } else {
                let frame_score = score + self.throws.get(idx + 1).unwrap_or(&0);
                total_score += frame_score;

                //Spare
                if frame_score == 10 {
                    total_score += self.throws.get(idx + 2).unwrap_or(&0);
                }

                idx += 2;
            }

            frame_idx += 1;
        }

        Some(total_score)
    }

    fn is_game_complete(&self) -> bool {
        self.current_frame > 10 && self.bonus_throw == 0
    }
}
