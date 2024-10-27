mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_squares(matrix: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(matrix);
    let mut dp = vec![vec![0; 1 + cols]; 1 + rows];
    for (y, r) in matrix.iter().enumerate() {
        for (x, &n) in r.iter().enumerate() {
            if n == 1 {
                dp[1 + y][1 + x] = 1 + dp[y][x].min(dp[y + 1][x]).min(dp[y][x + 1]);
            }
        }
    }
    dp.into_iter().flatten().sum()
}

fn top_down(matrix: &[&[i32]]) -> i32 {
    fn solve(matrix: &[&[i32]], dp: &mut [Vec<i32>], row: usize, col: usize) -> i32 {
        if matrix.get(row).is_none_or(|r| r.get(col).is_none()) {
            return 0;
        }
        if matrix[row][col] == 0 {
            return 0;
        }
        if dp[row][col] > -1 {
            return dp[row][col];
        }
        let right = solve(matrix, dp, row, 1 + col);
        let down = solve(matrix, dp, 1 + row, col);
        let diag = solve(matrix, dp, 1 + row, 1 + col);
        dp[row][col] = 1 + right.min(down).min(diag);
        dp[row][col]
    }
    let (rows, cols) = get_dimensions(matrix);
    let mut dp = vec![vec![-1; cols]; rows];
    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            res += solve(matrix, &mut dp, r, c);
        }
    }
    res // Well dp still has -1 in it
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(top_down(&[&[0, 1, 1, 1], &[1, 1, 1, 1], &[0, 1, 1, 1]]), 15);
        debug_assert_eq!(top_down(&[&[1, 0, 1], &[1, 1, 0], &[1, 1, 0]]), 7);
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
