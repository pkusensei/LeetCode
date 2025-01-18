mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = nums.len();
    let mut pow = vec![1; n];
    for i in 1..n {
        pow[i] = 2 * pow[i - 1] % MOD;
    }
    nums.sort_unstable();
    let mut res = 0;
    let [mut left, mut right] = [0, n - 1];
    while left <= right {
        if nums[left] + nums[right] > target {
            let Some(v) = right.checked_sub(1) else {
                break;
            };
            right = v;
        } else {
            res += pow[right - left];
            res %= MOD;
            left += 1;
        }
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
    fn test() {
        assert_eq!(num_subseq(vec![1], 1), 0);
        assert_eq!(
            num_subseq(
                vec![
                    27, 21, 14, 2, 15, 1, 19, 8, 12, 24, 21, 8, 12, 10, 11, 30, 15, 18, 28, 14, 26,
                    9, 2, 24, 23, 11, 7, 12, 9, 17, 30, 9, 28, 2, 14, 22, 19, 19, 27, 6, 15, 12,
                    29, 2, 30, 11, 20, 30, 21, 20, 2, 22, 6, 14, 13, 19, 21, 10, 18, 30, 2, 20, 28,
                    22
                ],
                31
            ),
            688052206
        );
    }

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
