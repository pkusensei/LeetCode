mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn add_negabinary(arr1: &mut [i32], arr2: &mut [i32]) -> Vec<i32> {
    arr1.reverse();
    arr2.reverse();
    let mut carry = 0;
    let n = arr1.len().max(arr2.len());
    let mut res = vec![];
    for i in 0..n {
        if let Some(v) = arr1.get(i) {
            carry += v;
        }
        if let Some(v) = arr2.get(i) {
            carry += v;
        }
        res.push(carry & 1);
        carry = -(carry >> 1);
    }
    while carry != 0 {
        res.push(carry & 1);
        carry = -(carry >> 1);
    }
    while res.last().is_some_and(|&v| v == 0) {
        res.pop();
    }
    if res.is_empty() {
        vec![0]
    } else {
        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            add_negabinary(&mut [1, 1, 1, 1, 1], &mut [1, 0, 1]),
            [1, 0, 0, 0, 0]
        );
        debug_assert_eq!(add_negabinary(&mut [0], &mut [0]), [0]);
        debug_assert_eq!(add_negabinary(&mut [0], &mut [1]), [1])
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
