mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    // let mut res = Vec::with_capacity(1 << n);
    // let mut idx = 0;
    // for (i, x) in (0..(1 << n)).enumerate() {
    //     let v = x ^ (x >> 1);
    //     res.push(v);
    //     if v == start {
    //         idx = i;
    //     }
    // }
    // res.rotate_left(idx);
    // res
    (0..(1 << n)).map(|i| start ^ i ^ (i >> 1)).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(circular_permutation(2, 3), [3, 2, 0, 1]);
        assert_eq!(circular_permutation(3, 2), [2, 6, 7, 5, 4, 0, 1, 3]);
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
