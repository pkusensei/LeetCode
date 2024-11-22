mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let (rows, cols) = get_dimensions(&grid);
        let [row, col] = [row, col].map(|v| v as usize);
        let mark = grid[row][col];
        let mut seen = vec![vec![false; cols]; rows];
        seen[row][col] = true;
        let mut front = HashSet::new();
        let mut queue = VecDeque::from([[row, col]]);
        while let Some([r, c]) = queue.pop_front() {
            if r == 0 || r == rows - 1 || c == 0 || c == cols - 1 {
                front.insert([r, c]);
            }
            for (nr, nc) in neighbors((r, c)) {
                match grid.get(nr).and_then(|row| row.get(nc)) {
                    None => {
                        front.insert([r, c]);
                    }
                    Some(&v) if v == mark && !seen[nr][nc] => {
                        queue.push_back([nr, nc]);
                        seen[nr][nc] = true;
                    }
                    Some(&v) if v == mark => (),
                    _ => {
                        front.insert([r, c]);
                    }
                }
            }
        }
        for [r, c] in front.into_iter() {
            grid[r][c] = color;
        }
        grid
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            color_border(vec![vec![1, 1], vec![1, 2]], 0, 0, 3),
            [[3, 3], [3, 2]]
        );
        debug_assert_eq!(
            color_border(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3),
            [[1, 3, 3], [2, 3, 3]]
        );
        debug_assert_eq!(
            color_border(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2),
            [[2, 2, 2], [2, 1, 2], [2, 2, 2]]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
