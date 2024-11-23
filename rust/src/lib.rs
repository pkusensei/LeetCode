mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rotate_the_box(grid: &mut [&mut [char]]) -> Vec<Vec<char>> {
    for row in grid.iter_mut() {
        for section in row.split_mut(|&ch| ch == '*') {
            let n = section.len();
            let count = section.iter().filter(|&&ch| ch == '#').count();
            section.fill('#');
            section[..n - count].fill('.');
        }
    }
    let (rows, cols) = get_dimensions(grid);
    let mut res = vec![vec!['.'; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            res[c][r] = grid[rows - 1 - r][c]
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
        debug_assert_eq!(
            rotate_the_box(&mut [&mut ['#', '.', '#']]),
            [['.'], ['#'], ['#']]
        );
        debug_assert_eq!(
            rotate_the_box(&mut [&mut ['#', '.', '*', '.'], &mut ['#', '#', '*', '.']]),
            [['#', '.'], ['#', '#'], ['*', '*'], ['.', '.']]
        );
        debug_assert_eq!(
            rotate_the_box(&mut [
                &mut ['#', '#', '*', '.', '*', '.'],
                &mut ['#', '#', '#', '*', '.', '.'],
                &mut ['#', '#', '#', '.', '#', '.']
            ]),
            [
                ['.', '#', '#'],
                ['.', '#', '#'],
                ['#', '#', '*'],
                ['#', '*', '.'],
                ['#', '.', '*'],
                ['#', '.', '.']
            ]
        )
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
