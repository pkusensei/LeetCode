mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reordered_power_of2(n: i32) -> bool {
    const ALL: [[u8; 10]; 31] = preprocess();
    let x = get_digits(n);
    ALL.iter().any(|&v| v == x)
}

const fn preprocess() -> [[u8; 10]; 31] {
    let mut res = [[0; 10]; 31];
    let mut x = 0;
    while x < 31 {
        res[x as usize] = get_digits(2i32.pow(x));
        x += 1;
    }
    res
}

const fn get_digits(mut n: i32) -> [u8; 10] {
    let mut res = [0; 10];
    while n > 0 {
        res[(n % 10) as usize] += 1;
        n /= 10;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(reordered_power_of2(1));
        debug_assert!(!reordered_power_of2(10));
        debug_assert!(reordered_power_of2(46));
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
