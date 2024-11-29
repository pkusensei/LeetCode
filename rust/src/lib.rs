mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time(grid: &[&[i32]]) -> i32 {
    if grid[0][1] > 1 && grid[1][0] > 1 {
        return -1;
    }
    let (rows, cols) = get_dimensions(grid);
    let mut heap = BinaryHeap::from([(Reverse(0), 0, 0)]);
    let mut seen = vec![vec![false; cols]; rows];
    seen[0][0] = true;
    while let Some((Reverse(t), x, y)) = heap.pop() {
        if x == cols - 1 && y == rows - 1 {
            return t;
        }
        for (nx, ny) in neighbors((x, y)) {
            if let Some(&v) = grid.get(ny).and_then(|r| r.get(nx)) {
                if seen[ny][nx] {
                    continue;
                }
                seen[ny][nx] = true;
                let wait = (v - (1 + t)) & 1;
                let time = (1 + t).max(v + wait);
                heap.push((Reverse(time), nx, ny));
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            minimum_time(&[&[0, 1, 3, 2], &[5, 1, 2, 5], &[4, 3, 8, 6]]),
            7
        );
        debug_assert_eq!(minimum_time(&[&[0, 2, 4], &[3, 2, 1], &[1, 0, 4]]), -1)
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
