mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = get_dimensions(&mat);
    let mask = to_num(mat);
    let mut queue = VecDeque::from([(mask, 0)]);
    let mut seen = HashSet::from([mask]);
    while let Some((mask, step)) = queue.pop_front() {
        if mask.count_ones() == 0 {
            return step;
        }
        for row in 0..rows {
            for col in 0..cols {
                let mut next = mask;
                let idx = row * cols + col;
                next ^= 1 << idx;
                for (nr, nc) in neighbors((row, col)) {
                    if (0..rows).contains(&nr) && (0..cols).contains(&nc) {
                        let i = nr * cols + nc;
                        next ^= 1 << i;
                    }
                }
                if seen.insert(next) {
                    queue.push_back((next, 1 + step));
                }
            }
        }
    }
    -1
}

fn to_num(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for r in grid {
        for num in r {
            res <<= 1;
            res |= num;
        }
    }
    res
}

fn to_grid(mut num: i32, rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut res = vec![];
    for _ in 0..rows {
        let mut curr = Vec::with_capacity(cols);
        for _ in 0..cols {
            curr.push(num & 1);
            num >>= 1;
        }
        curr.reverse();
        res.push(curr);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
        assert_eq!(min_flips(vec![vec![0]]), 0);
        assert_eq!(min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
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
