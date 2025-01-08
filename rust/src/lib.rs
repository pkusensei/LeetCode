mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(grid: &[&[i32]]) -> i32 {
        let (rows, cols) = get_dimensions(grid);
        let mut costs = HashMap::new();
        for r in 0..rows {
            for c in 0..cols {
                costs.insert([r, c], i32::MAX);
            }
        }
        costs.insert([0, 0], 0);
        let mut heap = BinaryHeap::from([(Reverse(0), [0, 0])]);
        while let Some((Reverse(cost), [row, col])) = heap.pop() {
            if row == rows - 1 && col == cols - 1 {
                return cost;
            }
            if costs[&[row, col]] < cost {
                continue;
            }
            let dir = grid[row][col];
            for (nr, nc) in neighbors((row, col)).filter(|&(nr, nc)| nr < rows && nc < cols) {
                let new_cost = cost
                    + if row == nr {
                        if nc > col {
                            i32::from(dir != 1)
                        } else {
                            i32::from(dir != 2)
                        }
                    } else if nr > row {
                        i32::from(dir != 3)
                    } else {
                        i32::from(dir != 4)
                    };
                if new_cost < costs[&[nr, nc]] {
                    costs.insert([nr, nc], new_cost);
                    heap.push((Reverse(new_cost), [nr, nc]));
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
        assert_eq!(
            min_cost(&[&[1, 1, 1, 1], &[2, 2, 2, 2], &[1, 1, 1, 1], &[2, 2, 2, 2]]),
            3
        );
        assert_eq!(min_cost(&[&[1, 1, 3], &[3, 2, 2], &[1, 1, 4]]), 0);
        assert_eq!(min_cost(&[&[1, 2], &[4, 3]]), 1);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
