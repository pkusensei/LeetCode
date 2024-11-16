mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_even_after_queries(nums: &mut [i32], queries: &[[i32; 2]]) -> Vec<i32> {
    queries
        .iter()
        .map(|v| {
            let idx = v[1] as usize;
            nums[idx] += v[0];
            nums.iter().filter(|&&n| n & 1 == 0).sum()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            sum_even_after_queries(&mut [1, 2, 3, 4], &[[1, 0], [-3, 1], [-4, 0], [2, 3]]),
            [8, 6, 2, 4]
        );
        debug_assert_eq!(sum_even_after_queries(&mut [1], &[[4, 0]]), [0]);
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
