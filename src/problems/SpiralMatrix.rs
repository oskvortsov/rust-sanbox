pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let n = size as usize;
    let mut matrix = vec![vec![0u32; n]; n];

    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let (mut row, mut col, mut dir) = (0usize, 0usize, 0usize);

    for num in 1..=(size * size) {
        matrix[row][col] = num;

        let (dr, dc) = directions[dir];

        let next_row = row as isize + dr;
        let next_col = col as isize + dc;

        if next_row < 0 || next_row >= n as isize ||
            next_col < 0 || next_col >= n as isize ||
            matrix[next_row as usize][next_col as usize] != 0 {
            dir = (dir + 1) % 4;
        }

        let (dr, dc) = directions[dir];
        row = (row as isize + dr) as usize;
        col = (col as isize + dc) as usize;
    }

    matrix
}


/**
Для `size = 4` разберём пошагово:

**1. `iter::once(size)` → `[4]`**

Один элемент — первый шаг по спирали (самый длинный).

**2. `.chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))`**

`(1..4).rev()` → `[3, 2, 1]`
Каждое число повторяется дважды через `flat_map`:
`[3, 3, 2, 2, 1, 1]`

Итого: `[4, 3, 3, 2, 2, 1, 1]` — количество шагов в каждом отрезке спирали.

**3. `.flat_map(|steps| iter::repeat(movement.next().unwrap()).take(steps))`**

Каждый "шаг" разворачивается в `steps` повторений текущего вектора направления:


4 шага → (1,0)(1,0)(1,0)(1,0)   // вправо
3 шага → (0,1)(0,1)(0,1)         // вниз
3 шага → (-1,0)(-1,0)(-1,0)      // влево
2 шага → (0,-1)(0,-1)            // вверх


`movement.next().unwrap()` вызывается ровно 7 раз (по числу отрезков), `VECTORS.iter().cycle()` обеспечивает бесконечный цикл направлений.

**Визуально для 4x4:**


→→→↓
↑ →↓↓
↑↑←↓
↑←←←

Длины отрезков: `4, 3, 3, 2, 2, 1, 1` ✓
**/

pub fn spiral_matrix_1(size: usize) -> Vec<Vec<u32>> {
    use std::iter;
    const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut matrix = vec![vec![0; size]; size];
    let mut movement = VECTORS.iter().cycle();

    let (mut x, mut y, mut n) = (-1, 0, 1..);
    for (move_x, move_y) in iter::once(size)
        .chain(
            (1..size).rev().flat_map(|n| iter::repeat(n).take(2))
        )
        .flat_map(|steps| iter::repeat(movement.next().unwrap()).take(steps))
    {
        x += move_x;
        y += move_y;
        matrix[y as usize][x as usize] = n.next().unwrap();
    }
    matrix
}