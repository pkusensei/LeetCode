mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_end(n: i32, x: i32) -> i64 {
    // fit bits of (n-1) onto "empty" bits of x
    // as if counting up from x for 0..n times
    let mut bits = format!("{:b}", x).into_bytes();
    let fill = format!("{:b}", n - 1).into_bytes();
    let mut idx = bits.len() - 1;
    for b in fill.into_iter().rev() {
        while idx > 0 && bits[idx] != b'0' {
            idx -= 1;
        }
        if idx > 0 {
            bits[idx] = b;
            idx -= 1;
        } else {
            bits.insert(0, b);
        }
    }
    let mut res = 0;
    for b in bits.into_iter() {
        res <<= 1;
        res |= i64::from(b - b'0');
    }
    res
}

fn consecutive_or(n: i32, x: i32) -> i64 {
    let mut res = i64::from(x);
    for _ in 1..n {
        res = (res + 1) | i64::from(x);
    }
    res
}

fn bitmask(n: i32, x: i32) -> i64 {
    let mut res: i64 = x.into();
    let mut mask: i64 = 1;
    let mut n: i64 = (n - 1).into();
    while n > 0 {
        if mask & i64::from(x) == 0 {
            res |= (n & 1) * mask;
            n >>= 1;
        }
        mask <<= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(bitmask(3, 4), 6);
        debug_assert_eq!(bitmask(2, 7), 15);
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
