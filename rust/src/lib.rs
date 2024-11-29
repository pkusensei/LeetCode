mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn corp_flight_bookings(bookings: &[[i32; 3]], n: i32) -> Vec<i32> {
    let mut res = vec![0; n as usize];
    for b in bookings.iter() {
        for idx in (b[0]..=b[1]).map(|v| v as usize - 1) {
            res[idx] += b[2];
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
        debug_assert_eq!(
            corp_flight_bookings(&[[1, 2, 10], [2, 3, 20], [2, 5, 25]], 5),
            [10, 55, 45, 25, 25]
        );
        debug_assert_eq!(corp_flight_bookings(&[[1, 2, 10], [2, 2, 15]], 2), [10, 25]);
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
