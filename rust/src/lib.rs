mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: &[i32]) -> Vec<Vec<i32>> {
    if upper + lower != colsum.iter().sum() {
        return vec![];
    }
    let n = colsum.len();
    let mut res = vec![vec![0; n]; 2];
    for i in colsum
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v == 2 { Some(i) } else { None })
    {
        upper -= 1;
        lower -= 1;
        if upper < 0 || lower < 0 {
            return vec![];
        }
        res[0][i] = 1;
        res[1][i] = 1;
    }
    for i in colsum
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v == 1 { Some(i) } else { None })
    {
        if upper > 0 {
            upper -= 1;
            res[0][i] = 1;
            continue;
        }
        if lower > 0 {
            lower -= 1;
            res[1][i] = 1;
            continue;
        }
        return vec![];
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(reconstruct_matrix(2, 1, &[1, 1, 1]), [[1, 1, 0], [0, 0, 1]]);
        assert!(reconstruct_matrix(2, 3, &[2, 2, 1, 1]).is_empty());
        assert_eq!(
            reconstruct_matrix(5, 5, &[2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
            [
                [1, 1, 1, 0, 1, 0, 0, 1, 0, 0],
                [1, 0, 1, 0, 0, 0, 1, 1, 0, 1]
            ]
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
            "left = {a:?}, right = {b:?}",
        );
    }
}
