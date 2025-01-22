mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_triplets(nums1: &[i32], nums2: &[i32]) -> i32 {
    let mut res = 0;
    for &num in nums1.iter() {
        res += find(nums2, i64::from(num).pow(2));
    }
    for &num in nums2.iter() {
        res += find(nums1, i64::from(num).pow(2));
    }
    res
}

fn find(nums: &[i32], target: i64) -> i32 {
    let mut res = 0;
    let mut seen = std::collections::HashMap::new();
    for &num in nums {
        let v = i64::from(num);
        if target % v > 0 {
            continue;
        }
        if let Some(v) = seen.get(&(target / v)) {
            res += v;
        }
        *seen.entry(v).or_insert(0) += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
