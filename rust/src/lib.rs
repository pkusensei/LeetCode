mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn process_queries(queries: &[i32], m: i32) -> Vec<i32> {
    let mut perm: Vec<_> = (1..=m).collect();
    let mut res = Vec::with_capacity(queries.len());
    for &q in queries.iter() {
        let Some(i) = perm.iter().position(|&v| v == q) else {
            continue;
        };
        res.push(i as i32);
        perm[0..=i].rotate_right(1);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(process_queries(&[3, 1, 2, 1], 5), [2, 1, 2, 1])
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
