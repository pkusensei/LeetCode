mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest1_bordered_square(grid: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(grid);
    let mut left = vec![vec![0; cols]; rows];
    let mut top = left.clone();
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == 1 {
                left[r][c] = v + if c > 0 { left[r][c - 1] } else { 0 };
                top[r][c] = v + if r > 0 { top[r - 1][c] } else { 0 };
            }
        }
    }
    for len in (1..=rows.min(cols)).rev() {
        for row in 0..=rows - len {
            for col in 0..=cols - len {
                if top[row + len - 1][col] >= len as i32
                    && top[row + len - 1][col + len - 1] >= len as i32
                    && left[row][col + len - 1] >= len as i32
                    && left[row + len - 1][col + len - 1] >= len as i32
                {
                    return (len * len) as i32;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            largest1_bordered_square(&[&[1, 1, 1], &[1, 0, 1], &[1, 1, 1]]),
            9
        );
        debug_assert_eq!(largest1_bordered_square(&[&[1, 1, 0, 0]]), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            largest1_bordered_square(&[&[1, 1, 1], &[1, 1, 0], &[1, 1, 1], &[0, 1, 1], &[1, 1, 1]]),
            4
        );
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
