mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn superpalindromes_in_range(left: &str, right: &str) -> i32 {
        const ROOTS: [u64; 70] = [
            1, 2, 3, 11, 22, 101, 111, 121, 202, 212, 1001, 1111, 2002, 10001, 10101, 10201, 11011,
            11111, 11211, 20002, 20102, 100001, 101101, 110011, 111111, 200002, 1000001, 1001001,
            1002001, 1010101, 1011101, 1012101, 1100011, 1101011, 1102011, 1110111, 1111111, 2000002,
            2001002, 10000001, 10011001, 10100101, 10111101, 11000011, 11011011, 11100111, 11111111,
            20000002, 100000001, 100010001, 100020001, 100101001, 100111001, 100121001, 101000101,
            101010101, 101020101, 101101101, 101111101, 110000011, 110010011, 110020011, 110101011,
            110111011, 111000111, 111010111, 111101111, 111111111, 200000002, 200010002,
        ];
        let a = left.parse().unwrap();
        let b = right.parse().unwrap();
        ROOTS
            .into_iter()
            .filter(|n| (a..=b).contains(&(n * n)))
            .count() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(superpalindromes_in_range("4", "1000"), 4);
        debug_assert_eq!(superpalindromes_in_range("1", "2"), 1);
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
