mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ship_within_days(weights: &[i32], days: i32) -> i32 {
        let mut left = weights.iter().copied().max().unwrap_or(weights[0]);
        let mut right: i32 = weights.iter().sum();
        while left < right {
            let mid = left + (right - left) / 2;
            if count(weights, mid) > days {
                left = 1 + mid;
            } else {
                right = mid
            }
        }
        left
}

fn count(weights: &[i32], mid: i32) -> i32 {
    let (mut res, mut sum) = (0, 0);
    for &num in weights.iter() {
        if sum + num <= mid {
            sum += num;
        } else {
            sum = num;
            res += 1
        }
    }
    res + i32::from(sum > 0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(ship_within_days(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
        debug_assert_eq!(ship_within_days(&[3, 2, 2, 4, 1, 4], 3), 6);
        debug_assert_eq!(ship_within_days(&[1, 2, 3, 1, 1], 4), 3);
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
