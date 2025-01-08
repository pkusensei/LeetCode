mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn closest_divisors(num: i32) -> Vec<i32> {
    let [a, b] = find(num + 1);
    let [c, d] = find(num + 2);
    if a.abs_diff(b) <= c.abs_diff(d) {
        vec![a, b]
    } else {
        vec![c, d]
    }
}

fn find(num: i32) -> [i32; 2] {
    let sq = f64::from(num).sqrt().floor() as i32;
    for a in (1..=sq).rev() {
        if num % a == 0 {
            return [a, num / a];
        }
    }
    [1, num]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(closest_divisors(8), [3, 3]);
        assert_eq!(closest_divisors(123), [5, 25]);
        assert_eq!(closest_divisors(999), [25, 40]);
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
