mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    const SEQ: &str = "123456789";
    let n1 = 1 + low.ilog10() as usize;
    let n2 = (1 + high.ilog10() as usize).min(9);
    let mut res = vec![];
    for len in n1..=n2 {
        for i in 0..=9 - len {
            let v = SEQ[i..i + len].parse::<i32>().unwrap();
            if (low..=high).contains(&v) {
                res.push(v);
            }
        }
    }
    res.sort_unstable();
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(sequential_digits(100, 300), [123, 234]);
        assert_eq!(
            sequential_digits(1000, 13000),
            [1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            sequential_digits(10, 1000000000),
            [
                12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678, 789, 1234, 2345,
                3456, 4567, 5678, 6789, 12345, 23456, 34567, 45678, 56789, 123456, 234567, 345678,
                456789, 1234567, 2345678, 3456789, 12345678, 23456789, 123456789
            ]
        )
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
