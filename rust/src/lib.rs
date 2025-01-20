mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn first_complete_index(arr: &[i32], mat: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(mat);
    let mut grid = std::collections::HashMap::new();
    for (r, row) in mat.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            grid.insert(v, [r, c]);
        }
    }
    let mut rcount = vec![0; rows];
    let mut ccount = vec![0; cols];
    for (i, num) in arr.iter().enumerate() {
        let [r, c] = grid[num];
        rcount[r] += 1;
        ccount[c] += 1;
        if rcount[r] == cols || ccount[c] == rows {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(first_complete_index(&[1, 3, 4, 2], &[&[1, 4], &[2, 3]]), 2);
        assert_eq!(
            first_complete_index(
                &[2, 8, 7, 4, 1, 3, 5, 6, 9],
                &[&[3, 2, 5], &[1, 4, 6], &[8, 7, 9]]
            ),
            3
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
