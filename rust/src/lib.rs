mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let [a, b, n] = [a, b, n].map(i64::from);
    let mut left = a.min(b);
    let mut right = a.max(b) * n;
    let lcm = a * b / gcd(a, b);
    while left <= right {
        let mut mid = left + (right - left) / 2;
        let count = mid / a + mid / b - mid / lcm;
        match count.cmp(&n) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
            std::cmp::Ordering::Equal => {
                while mid % a > 0 && mid % b > 0 {
                    mid -= 1;
                }
                return (mid % MOD) as i32;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(nth_magical_number(1, 2, 3), 2);
        debug_assert_eq!(nth_magical_number(4, 2, 3), 6);
    }

    #[test]
    fn test() {
        debug_assert_eq!(nth_magical_number(8, 8, 8), 64);
    }

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
