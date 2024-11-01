mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_eating_speed(piles: &[i32], h: i32) -> i32 {
    let (mut left, mut right) = (1, piles.iter().copied().max().unwrap());
    while left < right {
        let mid = left + (right - left) / 2;
        let t = calc(piles, mid);
        if t > h {
            left = mid + 1
        } else {
            right = mid;
        }
    }
    left
}

fn calc(nums: &[i32], x: i32) -> i32 {
    nums.iter()
        .map(|&num| num / x + i32::from(num % x > 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_eating_speed(&[3, 6, 7, 11], 8), 4);
        debug_assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 5), 30);
        debug_assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 6), 23);
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
