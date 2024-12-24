mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_falling_path_sum(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    if n == 1 {
        return grid[0][0];
    }
    let [(mut i1, mut n1), (mut _i2, mut n2)] = find_2_mins(grid[0]);
    for row in grid.iter().skip(1) {
        let mut dp = Vec::with_capacity(n);
        for (idx, &num) in row.iter().enumerate() {
            if idx == i1 {
                dp.push(num + n2);
            } else {
                dp.push(num + n1);
            }
        }
        [(i1, n1), (_i2, n2)] = find_2_mins(&dp);
    }
    n1
}

fn find_2_mins(row: &[i32]) -> [(usize, i32); 2] {
    let (mut i1, mut n1) = (0, i32::MAX);
    let (mut i2, mut n2) = (0, i32::MAX);
    for (idx, &num) in row.iter().enumerate() {
        if num < n1 {
            i2 = i1;
            n2 = n1;
            n1 = num;
            i1 = idx
        } else if num < n2 {
            n2 = num;
            i2 = idx;
        }
    }
    [(i1, n1), (i2, n2)]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            min_falling_path_sum(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]),
            13
        );
        assert_eq!(min_falling_path_sum(&[&[7]]), 7);
    }

    #[test]
    fn test() {
        assert_eq!(
            min_falling_path_sum(&[
                &[50, -18, -38, 39, -20, -37, -61, 72, 22, 79],
                &[82, 26, 30, -96, -1, 28, 87, 94, 34, -89],
                &[55, -50, 20, 76, -50, 59, -58, 85, 83, -83],
                &[39, 65, -68, 89, -62, -53, 74, 2, -70, -90],
                &[1, 57, -70, 83, -91, -32, -13, 49, -11, 58],
                &[-55, 83, 60, -12, -90, -37, -36, -27, -19, -6],
                &[76, -53, 78, 90, 70, 62, -81, -94, -32, -57],
                &[-32, -85, 81, 25, 80, 90, -24, 10, 27, -55],
                &[39, 54, 39, 34, -45, 17, -2, -61, -81, 85],
                &[-77, 65, 76, 92, 21, 68, 78, -13, 39, 22]
            ]),
            -807
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
