use std::collections::VecDeque;

pub fn regions_by_slashes(grid: &[&str]) -> i32 {
    let size = grid.len();
    let mut expanded = vec![vec![0; size * 3]; size * 3];
    for (y, row) in grid.iter().enumerate() {
        for (x, b) in row.bytes().enumerate() {
            let (base_row, base_col) = (y * 3, x * 3);
            if b == b'\\' {
                expanded[base_row][base_col] = 1;
                expanded[base_row + 1][base_col + 1] = 1;
                expanded[base_row + 2][base_col + 2] = 1;
            } else if b == b'/' {
                expanded[base_row][base_col + 2] = 1;
                expanded[base_row + 1][base_col + 1] = 1;
                expanded[base_row + 2][base_col] = 1;
            }
        }
    }

    let mut res = 0;
    for row in 0..size * 3 {
        for col in 0..size * 3 {
            if expanded[row][col] == 0 {
                floodfill(row, col, &mut expanded);
                res += 1;
            }
        }
    }
    res
}

fn floodfill(row: usize, col: usize, expanded: &mut [Vec<u8>]) {
    let mut queue = VecDeque::from([(row, col)]);
    expanded[row][col] = 1;
    while let Some(curr) = queue.pop_front() {
        for neighbor in neighbors(curr) {
            if check(neighbor, expanded) {
                expanded[neighbor.0][neighbor.1] = 1;
                queue.push_back(neighbor);
            }
        }
    }
}

fn check((row, col): (usize, usize), expanded: &[Vec<u8>]) -> bool {
    let size = expanded.len();
    (0..size).contains(&row) && (0..size).contains(&col) && expanded[row][col] == 0
}

fn neighbors((a, b): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [
        (a.saturating_sub(1), b),
        (a + 1, b),
        (a, b.saturating_sub(1)),
        (a, b + 1),
    ]
    .into_iter()
    .filter(move |p| *p != (a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(regions_by_slashes(&[" /", "/ "]), 2);
        debug_assert_eq!(regions_by_slashes(&[" /", "  "]), 1);
        debug_assert_eq!(regions_by_slashes(&["/\\", "\\/"]), 5);
    }

    #[test]
    fn test() {}
}
