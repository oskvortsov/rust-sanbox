use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result:Vec<(usize, usize)> = vec![];

    let row_size = input.len();
    let col_size = input[0].len();

    let mut row_max: HashMap<usize, u64> = HashMap::with_capacity(row_size);
    let mut col_min: HashMap<usize, u64> = HashMap::with_capacity(col_size);

    (0..row_size).for_each(|row| { row_max.insert(row, 0); });
    (0..col_size).for_each(|col| { col_min.insert(col, u64::MAX); });

    (0..row_size).for_each(|row| {
        (0..col_size).for_each(|col| {
            let cur = input[row][col];

            if row_max.get(&row) < Some(&cur) {
                row_max.insert(row, cur);
            }

            if col_min.get(&col) > Some(&cur) {
                col_min.insert(col, cur);
            }
        })
    });

    (0..row_size).for_each(|row| {
        (0..col_size).for_each(|col| {
            let cur = Some(&input[row][col]);

            if row_max.get(&row) == cur && col_min.get(&col) == cur {
                result.push((row, col));
            };
        });
    });

    result
}
