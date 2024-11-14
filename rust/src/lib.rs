mod dsu;
mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn k_closest(points: &[[i32; 2]], k: i32) -> Vec<Vec<i32>> {
    let mut queue: BinaryHeap<_> = points.iter().map(|v| Point { p: [v[0], v[1]] }).collect();
    while queue.len() > k as usize {
        queue.pop();
    }
    queue.into_iter().map(|v| v.p.to_vec()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    p: [i32; 2],
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.p[0].pow(2) + self.p[1].pow(2)).cmp(&(other.p[0].pow(2) + other.p[1].pow(2)))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(k_closest(&[[1, 3], [-2, 2]], 1), vec![vec![-2, 2]]);
        sort_eq(
            k_closest(&[[3, 3], [5, -1], [-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]],
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
