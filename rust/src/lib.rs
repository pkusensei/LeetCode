mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn consecutive_numbers_sum(n: i32) -> i32 {
    // n = a + (a+1) + (a+2) + .. + (a+k)
    // n = a*(1+k) + (1+2+..k)
    let mut res = 0;
    let mut k = 0;
    while k * (1 + k) / 2 < n {
        let delta = n - k * (1 + k) / 2;
        if delta % (1 + k) == 0 {
            res += 1;
        }
        k += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(consecutive_numbers_sum(5), 2);
        debug_assert_eq!(consecutive_numbers_sum(9), 3);
        debug_assert_eq!(consecutive_numbers_sum(15), 4);
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
