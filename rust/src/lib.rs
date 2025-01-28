mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut seen = vec![vec![false; cols]; rows];
    let mut res = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v > 0 && !seen[r][c] {
                res = res.max(bfs(&grid, &mut seen, [r, c]));
            }
        }
    }
    res
}

fn bfs(grid: &[Vec<i32>], seen: &mut [Vec<bool>], start: [usize; 2]) -> i32 {
    let mut res = 0;
    let mut queue = VecDeque::from([(start, grid[start[0]][start[1]])]);
    seen[start[0]][start[1]] = true;
    while let Some(([r, c], count)) = queue.pop_front() {
        res += count;
        for [nr, nc] in neighbors([r, c]) {
            if let Some(&v) = grid.get(nr).and_then(|row| row.get(nc)) {
                if v > 0 && !seen[nr][nc] {
                    seen[nr][nc] = true;
                    queue.push_back(([nr, nc], v));
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
