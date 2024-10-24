mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_increase_keeping_skyline(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    if n < 2 {
        return 0;
    }
    let rows: Vec<i32> = grid
        .iter()
        .map(|v| v.iter().copied().max().unwrap_or(0))
        .collect();
    let cols: Vec<i32> = (0..n)
        .map(|i| grid.iter().map(|v| v[i]).max().unwrap_or(0))
        .collect();
    let mut res = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &n) in row.iter().enumerate() {
            res += rows[y].min(cols[x]) - n;
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
            max_increase_keeping_skyline(&[
                &[3, 0, 8, 4],
                &[2, 4, 5, 7],
                &[9, 2, 6, 3],
                &[0, 3, 1, 0]
            ]),
            35
        );
        debug_assert_eq!(
            max_increase_keeping_skyline(&[&[0, 0, 0], &[0, 0, 0], &[0, 0, 0]]),
            0
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
}
