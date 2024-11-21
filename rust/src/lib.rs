mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    for r in 0..rows {
        for c in 0..cols {
            res.push(vec![r, c]);
        }
    }
    res.sort_unstable_by_key(|v| v[0].abs_diff(r_center) + v[1].abs_diff(c_center));
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(all_cells_dist_order(1, 2, 0, 0), [[0, 0], [0, 1]]);
        debug_assert_eq!(
            all_cells_dist_order(2, 2, 0, 1),
            [[0, 1], [0, 0], [1, 1], [1, 0]]
        );
        debug_assert_eq!(
            all_cells_dist_order(2, 3, 1, 2),
            [[1, 2], [0, 2], [1, 1], [0, 1], [1, 0], [0, 0]]
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
