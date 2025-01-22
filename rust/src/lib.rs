mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut rmap = HashMap::new();
    let mut cmap = HashMap::new();
    let mut ps = vec![];
    for (r, row) in mat.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == 1 {
                *rmap.entry(r).or_insert(0) += 1;
                *cmap.entry(c).or_insert(0) += 1;
                ps.push([r, c]);
            }
        }
    }
    ps.into_iter()
        .filter(|[r, c]| rmap[r] == 1 && cmap[c] == 1)
        .count() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
