mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn max_performance(_n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
    let mut espairs: Vec<_> = efficiency
        .into_iter()
        .zip(speed)
        .map(|(e, s)| [e, s].map(i64::from))
        .collect();
    espairs.sort_unstable_by_key(|p| Reverse(p[0]));
    let mut sum = 0;
    let mut res = 0;
    let mut heap = BinaryHeap::new();
    for [ef, sp] in espairs {
        heap.push(Reverse(sp));
        sum += sp;
        if heap.len() > k as usize {
            let s = heap.pop().map(|r| r.0).unwrap_or(0);
            sum -= s;
        }
        res = res.max(sum * ef);
    }
    (res % 1_000_000_007) as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
            60
        );
        assert_eq!(
            max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
            68
        );
        assert_eq!(
            max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
            72
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
