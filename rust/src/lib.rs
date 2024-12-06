mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_count(banned: &[i32], n: i32, max_sum: i32) -> i32 {
    let banned: std::collections::HashSet<i32> =
        banned.iter().copied().filter(|&num| num <= n).collect();
    let mut res = 0;
    let mut sum = 0;
    for num in (1..=n).filter(|num| !banned.contains(num)) {
        sum += num;
        if sum <= max_sum {
            res += 1;
        } else {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_count(&[1, 6, 5], 5, 6), 2);
        assert_eq!(max_count(&[1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
        assert_eq!(max_count(&[11], 7, 50), 7);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
