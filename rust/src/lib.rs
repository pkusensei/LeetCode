mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn arrange_coins(n: i32) -> i32 {
    let (mut left, mut right) = (0, i64::from(n));
    while left <= right {
        let mid = left + (right - left) / 2;
        if (mid + 1) * mid / 2 <= i64::from(n) {
            left = mid + 1;
        } else {
            right = mid - 1
        }
    }
    left as i32 - 1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(arrange_coins(5), 2);
        debug_assert_eq!(arrange_coins(8), 3);
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
}
