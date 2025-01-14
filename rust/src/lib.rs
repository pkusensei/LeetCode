mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_diff(num: i32) -> i32 {
    let mut digits = vec![];
    let mut x = num;
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    digits.reverse();
    if digits.len() == 1 {
        8
    } else {
        let a = to_big(&digits);
        let b = to_small(digits);
        a - b
    }
}

fn to_small(digits: Vec<i32>) -> i32 {
    let (x, y) = if digits[0] > 1 {
        (digits[0], 1)
    } else {
        // digits[0] == 1
        let Some(&v) = digits.iter().find(|&&v| v > 1) else {
            return digits.iter().fold(0, |acc, d| acc * 10 + d);
        };
        (v, 0)
    };
    digits
        .into_iter()
        .fold(0, |acc, d| acc * 10 + if d == x { y } else { d })
}

fn to_big(digits: &[i32]) -> i32 {
    let Some(&v) = digits.iter().find(|&&v| v < 9) else {
        return digits.iter().fold(0, |acc, d| acc * 10 + d);
    };
    digits
        .iter()
        .fold(0, |acc, &d| acc * 10 + if d == v { 9 } else { d })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_diff(555), 888);
        assert_eq!(max_diff(9), 8);

        assert_eq!(max_diff(10000), 80000);
        assert_eq!(max_diff(9288), 8700);
        assert_eq!(max_diff(111), 888);
    }

    #[test]
    fn test() {
        assert_eq!(max_diff(123456), 820000);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
