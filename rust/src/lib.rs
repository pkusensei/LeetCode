mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn odd_cells(m: i32, n: i32, indices: &[[i32; 2]]) -> i32 {
    let [mut rows, mut cols] = [m, n].map(|v| vec![0; v as usize]);
    for v in indices.iter() {
        rows[v[0] as usize] += 1;
        cols[v[1] as usize] += 1;
    }
    let mut res = 0;
    for row in rows.into_iter() {
        for col in cols.iter() {
            res += (row + col) & 1;
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
        assert_eq!(odd_cells(2, 3, &[[0, 1], [1, 1]]), 6);
        assert_eq!(odd_cells(2, 2, &[[1, 1], [0, 0]]), 0);
    }

    #[test]
    fn test() {
        assert_eq!(odd_cells(48, 37, &[[40, 5]]), 83)
    }

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
