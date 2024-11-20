mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn base_neg2(mut n: i32) -> String {
    let mut res = vec![];
    while n != 0 {
        res.push(b'0' + (n & 1) as u8);
        n = -(n >> 1); // same thing as base2 except the minus
    }
    res.reverse();
    if res.is_empty() {
        "0".into()
    } else {
        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(base_neg2(2), "110");
        debug_assert_eq!(base_neg2(3), "111");
        debug_assert_eq!(base_neg2(4), "100");
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
