mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_range_query(nums: &mut [i32], requests: &[[i32; 2]]) -> i32 {
    let n = nums.len();
    let mut freq = requests.iter().fold(vec![0; 1 + n], |mut acc, req| {
        acc[req[0] as usize] += 1;
        acc[req[1] as usize + 1] -= 1;
        acc
    });
    for i in 1..n {
        freq[i] += freq[i - 1];
    }
    nums.sort_unstable();
    freq[..n].sort_unstable();
    nums.iter().zip(freq).fold(0, |acc, (&num, f)| {
        (acc + i64::from(num) * f) % 1_000_000_007
    }) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            max_sum_range_query(&mut [1, 2, 3, 4, 5], &[[1, 3], [0, 1]]),
            19
        );
        assert_eq!(max_sum_range_query(&mut [1, 2, 3, 4, 5, 6], &[[0, 1]]), 11);
        assert_eq!(
            max_sum_range_query(&mut [1, 2, 3, 4, 5, 10], &[[0, 2], [1, 3], [1, 1]]),
            47
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
