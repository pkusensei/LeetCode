mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_subseq_widths(nums: &mut [i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let mut prefix = Vec::with_capacity(n);
    prefix.push(1);
    for _ in 1..n {
        prefix.push(prefix.last().unwrap_or(&1) * 2 % MOD);
    }
    // min max has nothing to do with sorting
    nums.sort_unstable();
    // Array [..,i] has subseq with count of 2^i; non-empty 2^i -1
    // Its count is ( 2^i -1 )*([i] - [0])
    // Array [1+i..] has 2^(n-1-i)
    // Similarly it's ( 2^(n-1-i) -1 ) * ([n-1] - [i])
    // Thus for [i], its contribution is ( (2^i -1) - (2^(n-1-i) -1) ) * [i]
    // The two (-1)s cancel out
    let mut res = 0;
    for (i, &num) in nums.iter().enumerate() {
        res = (res + (prefix[i] - prefix[n - 1 - i]) * i64::from(num)) % MOD;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(sum_subseq_widths(&mut [2, 1, 3]), 6);
        debug_assert_eq!(sum_subseq_widths(&mut [2]), 0);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
