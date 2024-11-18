mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn decrypt(code: &[i32], k: i32) -> Vec<i32> {
    let n = code.len();
    if k == 0 {
        return vec![0; n];
    }
    let mut res = Vec::with_capacity(n);
    for idx in 0..n as i32 {
        let v = if k > 0 {
            idx + 1..idx + k + 1
        } else {
            idx + k..idx
        }
        .map(|i| code[i.rem_euclid(n as i32) as usize])
        .sum();
        res.push(v);
    }
    res

    // if k < 0 {
    //     code.reverse();
    //     k = -k;
    //     (0..code.len()).map(|i| code.iter().cycle().skip(i+1).take(k as usize).sum()).rev().collect()
    // } else {
    //     (0..code.len()).map(|i| code.iter().cycle().skip(i+1).take(k as usize).sum()).collect()
    // }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(decrypt(&[5, 7, 1, 4], 3), [12, 10, 16, 13]);
        debug_assert_eq!(decrypt(&[1, 2, 3, 4], 0), [0, 0, 0, 0]);
        debug_assert_eq!(decrypt(&[2, 4, 9, 3], -2), [12, 5, 6, 13]);
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
