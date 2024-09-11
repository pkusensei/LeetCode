mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_perfect_square(num: i32) -> bool {
    let num = i64::from(num);
    let (mut left, mut right) = (0, num);
    while left <= right {
        // 2^30 * 2^30 overflows ==> TLE
        let mid = left + (right - left) / 2;
        match (mid * mid).cmp(&num) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_perfect_square(16));
        debug_assert!(!is_perfect_square(14));
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
