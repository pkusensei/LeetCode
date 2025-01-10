mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_number_of_families(n: i32, reserved_seats: &mut [[i32; 2]]) -> i32 {
    const TWO: i16 = 0b011_1111_110;
    const ONE: i16 = 0b000_1111_000;
    const LEFT: i16 = 0b011_1100_000;
    const RIGHT: i16 = 0b000_0011_110;
    reserved_seats.sort_unstable_by_key(|s| s[0]);
    let mut res = 2 * n;
    for row in reserved_seats.chunk_by(|a, b| a[0] == b[0]) {
        let mask = row.iter().fold(0, |acc, s| acc | 1 << (s[1] - 1));
        if mask & TWO == 0 {
        } else if mask & ONE == 0 || mask & LEFT == 0 || mask & RIGHT == 0 {
            res -= 1
        } else {
            res -= 2;
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
        assert_eq!(
            max_number_of_families(3, &mut [[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]]),
            4
        );
        assert_eq!(max_number_of_families(2, &mut [[2, 1], [1, 8], [2, 6]]), 2);
        assert_eq!(
            max_number_of_families(4, &mut [[4, 3], [1, 4], [4, 6], [1, 7]]),
            4
        );
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
