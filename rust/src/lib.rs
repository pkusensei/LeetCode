mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_four_divisors(nums: &[i32]) -> i32 {
    nums.iter().filter_map(|&num| solve(num)).sum()
}

fn solve(num: i32) -> Option<i32> {
    let sq = f64::from(num).sqrt().floor() as i32;
    let mut factors = Vec::with_capacity(2);
    for v in 2..=sq {
        if num % v == 0 {
            if factors.is_empty() && v * v < num {
                factors.extend([v, num / v]);
            } else {
                return None;
            }
        }
    }
    if factors.len() == 2 {
        Some(factors.into_iter().sum::<i32>() + num + 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(sum_four_divisors(&[21, 4, 7]), 32);
        assert_eq!(sum_four_divisors(&[21, 21]), 64);
        assert_eq!(sum_four_divisors(&[1, 2, 3, 4, 5]), 0);
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
