// https://exercism.org/tracks/rust/exercises/minesweeper

// topics #[BFS, casting, for]

use std::collections::VecDeque;

const EMPTY: char = ' ';
const MINE: char = '*';

const NEIGHBOURS_CELLS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let col_len = minefield.len() as i32;

    (0..col_len).map(|y| {
        let row_len = minefield[y as usize].len() as i32;

        (0..row_len).map(|x| {
            let current = minefield[y as usize].as_bytes()[x as usize] as char;

            if current == MINE {
                MINE
            } else {
                match NEIGHBOURS_CELLS
                    .iter()
                    .map(|&(nx, ny)| (x + nx, y + ny))
                    .filter(|&(dx, dy)| (0 <= dx && dx < row_len) && (0 <= dy && dy < col_len))
                    .filter(|&(dx, dy)| minefield[dy as usize].as_bytes()[dx as usize] as char == MINE)
                    .count()
                {
                    0 => EMPTY,
                    n => (n as u8 + '0' as u8) as char,
                }
            }
        })
            .collect()
    })
        .collect()
}

pub fn annotate_1(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let col_len = minefield.len();
    let row_len = minefield[0].len();
    let mut tmp: Vec<Vec<i32>> = vec![vec![0; row_len]; col_len];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for x in 0..col_len {
        for y in 0..row_len {
            if minefield[x].as_bytes()[y] == MINE as u8 {
                tmp[x][y] = -1;
                queue.push_back((x, y));
            }
        }
    }

    while !queue.is_empty() {
        let Some((x, y)) = queue.pop_back() else { continue };

        for (sx, sy) in STEPS {
            let dx = x as i32 + sx ;
            let dy = y as i32 + sy;

            if is_outbound(dx, dy, col_len, row_len) || tmp[dx as usize][dy as usize] == -1 {
                continue
            };

            tmp[dx as usize][dy as usize] += 1;
        }
    }

    convert_to_string(&tmp)
}

fn is_outbound(dx: i32, dy: i32, col_len: usize, row_len: usize) -> bool {
    dx < 0 || dy < 0 || dx as usize == col_len || dy as usize == row_len
}

fn convert_to_string(matrix: &Vec<Vec<i32>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for row in matrix {
        let mut row_string = String::new();

        for x in row {
            match x {
                -1 => row_string.push(MINE),
                0 => row_string.push(EMPTY),
                other => row_string.push_str(other.to_string().as_str())
            }
        }

        result.push(row_string);
    }

    result
}