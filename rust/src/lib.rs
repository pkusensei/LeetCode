mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let (mut left, mut right) = (0, n - 1);
    let (mut open, mut close) = (0, 0);
    let mut res = 0;
    while left < right {
        if s[left] == b'[' {
            open += 1;
        } else {
            close += 1
        }
        if close > open {
            while s[right] != b'[' {
                right -= 1;
            }
            close -= 1;
            open += 1;
            res += 1;
            right -= 1;
        }
        left += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(min_swaps("][]["), 1);
        debug_assert_eq!(min_swaps("]]][[["), 2);
        debug_assert_eq!(min_swaps("[]"), 0);
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
