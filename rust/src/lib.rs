mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn day_of_year(date: &str) -> i32 {
    const DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut it = date.splitn(3, '-');
    let y = it.next().and_then(|s| s.parse::<i32>().ok()).unwrap();
    let m = it.next().and_then(|s| s.parse::<i32>().ok()).unwrap();
    let d = it.next().and_then(|s| s.parse::<i32>().ok()).unwrap();
    let mut res = d;
    for i in 0..m - 1 {
        res += DAYS[i as usize];
    }
    if leap(y) && m > 2 {
        res += 1;
    }
    res
}

const fn leap(y: i32) -> bool {
    y % 400 == 0 || (y % 4 == 0 && y % 100 > 0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(day_of_year("2019-01-09"), 9);
        assert_eq!(day_of_year("2019-02-10"), 41);
    }

    #[test]
    fn test() {
        assert_eq!(day_of_year("2016-02-09"), 40);
        assert_eq!(day_of_year("1900-05-02"), 122);
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
