mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
    let [n, a, b, c] = [n, a, b, c].map(i64::from);
    let ab = a * b / gcd(a, b);
    let ac = a * c / gcd(a, c);
    let bc = b * c / gcd(c, b);
    let abc = ab * c / gcd(ab, c);
    let mut left = 1;
    let mut right = 2_000_000_000;
    while left < right {
        let mid = left + (right - left) / 2;
        let count = mid / a + mid / b + mid / c - mid / ab - mid / ac - mid / bc + mid / abc;
        if count < n {
            left = 1 + mid
        } else {
            right = mid
        }
    }
    left as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(nth_ugly_number(3, 2, 3, 5), 4);
        assert_eq!(nth_ugly_number(4, 2, 3, 4), 6);
        assert_eq!(nth_ugly_number(5, 2, 11, 13), 10);
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
