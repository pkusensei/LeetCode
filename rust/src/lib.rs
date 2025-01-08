mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn days_between_dates(date1: &str, date2: &str) -> i32 {
    let [d1, d2] = [date1, date2].map(parse);
    d1.abs_diff(d2) as _
}

const fn is_leap(y: i32) -> bool {
    y % 400 == 0 || (y % 4 == 0 && y % 100 > 0)
}

fn parse(s: &str) -> i32 {
    const DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut it = s.split('-');
    let y: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(1971);
    let m: usize = it.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let d: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let mut res = (1971..y).map(|i| 365 + i32::from(is_leap(i))).sum();
    res += DAYS.iter().take(m - 1).sum::<i32>();
    res += i32::from(is_leap(y) && m > 2);
    res += d;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(days_between_dates("2019-06-29", "2019-06-30"), 1);
        assert_eq!(days_between_dates("2020-01-15", "2019-12-31"), 15);
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
