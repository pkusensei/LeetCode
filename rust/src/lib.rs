mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_taps(n: i32, ranges: &[i32]) -> i32 {
    let n = n as usize;
    let mut arr = vec![0; 1 + n];
    for (idx, &r) in ranges.iter().enumerate() {
        if r == 0 {
            continue;
        }
        let left = (idx as i32 - r).max(0) as usize;
        arr[left] = arr[left].max(idx + r as usize);
    }
    let (mut end, mut reach) = (0, 0);
    let mut count = 0;
    for (idx, right) in arr.into_iter().enumerate() {
        if idx > end {
            if reach <= end {
                return -1;
            }
            end = reach;
            count += 1;
        }
        reach = reach.max(right);
    }
    count + i32::from(end < n)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_taps(5, &[3, 4, 1, 1, 0, 0]), 1);
        assert_eq!(min_taps(3, &[0, 0, 0, 0]), -1);
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
