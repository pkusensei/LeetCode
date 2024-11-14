mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subarrays_div_by_k(nums: &[i32], k: i32) -> i32 {
    let mut prefix = 0;
    let mut mod_groups = vec![0; k as usize];
    mod_groups[0] = 1;
    let mut res = 0;
    for &num in nums.iter() {
        prefix = (prefix + num.rem_euclid(k)).rem_euclid(k);
        // any mod that's seen again means
        // a pair of indices (i, j) creates a wanted subarray
        res += mod_groups[prefix as usize];
        mod_groups[prefix as usize] += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(subarrays_div_by_k(&[4, 5, 0, -2, -3, 1], 5), 7);
        debug_assert_eq!(subarrays_div_by_k(&[5], 9), 0);
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
