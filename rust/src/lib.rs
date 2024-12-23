mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn group_the_people(group_sizes: &[i32]) -> Vec<Vec<i32>> {
    let map = group_sizes.iter().enumerate().fold(
        std::collections::HashMap::<_, Vec<_>>::new(),
        |mut acc, (i, &size)| {
            acc.entry(size).or_default().push(i as i32);
            acc
        },
    );
    let mut res = vec![];
    for (k, v) in map {
        res.extend(v.chunks_exact(k as usize).map(|ch| ch.to_vec()));
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            group_the_people(&[3, 3, 3, 3, 3, 1, 3]),
            [vec![5], vec![0, 1, 2], vec![3, 4, 6]],
        );
        sort_eq(
            group_the_people(&[2, 1, 3, 3, 3, 2]),
            [vec![1], vec![0, 5], vec![2, 3, 4]],
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
