mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn xor_queries(arr: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(arr.len());
    prefix.push(arr[0]);
    for &num in arr.iter().skip(1) {
        prefix.push(num ^ prefix.last().unwrap());
    }
    let mut res = Vec::with_capacity(queries.len());
    for q in queries.iter() {
        let (a, b) = (q[0] as usize, q[1] as usize);
        if a > 0 {
            res.push(prefix[b] ^ prefix[a - 1]);
        } else {
            res.push(prefix[b]);
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
            xor_queries(&[1, 3, 4, 8], &[[0, 1], [1, 2], [0, 3], [3, 3]]),
            [2, 7, 14, 8]
        );
        assert_eq!(
            xor_queries(&[4, 8, 2, 10], &[[2, 3], [1, 3], [0, 0], [0, 3]]),
            [8, 0, 4, 4]
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
