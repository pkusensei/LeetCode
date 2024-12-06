mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sum(arr: &[i32]) -> i32 {
    let mut prev_delete = 0;
    let mut prev_keep = arr[0];
    let mut res = arr[0];
    for num in arr.iter().skip(1) {
        // attemp to skip current num
        prev_delete = (prev_delete + num).max(prev_keep);
        // add current in or start from current
        prev_keep = (prev_keep + num).max(*num);
        res = res.max(prev_delete).max(prev_keep);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(maximum_sum(&[1, -2, 0, 3]), 4);
        assert_eq!(maximum_sum(&[1, -2, -2, 3]), 3);
        assert_eq!(maximum_sum(&[-1, -1, -1, -1]), -1);
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
