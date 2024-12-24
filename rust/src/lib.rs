mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn remove_covered_intervals(intervals: &mut [[i32; 2]]) -> i32 {
    intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut prev = [-1, -1];
    let mut res = 0;
    for int in intervals.iter() {
        if prev[0] <= int[0] && int[1] <= prev[1] {
            continue;
        }
        prev = [int[0], int[1]];
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(remove_covered_intervals(&mut [[1, 4], [3, 6], [2, 8]]), 2);
        assert_eq!(remove_covered_intervals(&mut [[1, 4], [2, 3]]), 1);
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
