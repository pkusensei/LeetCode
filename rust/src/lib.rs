mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

const MOD: i32 = 1_000_000_007;

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    // dp[move][row][col]
    // 1) ^^^^  ^^^  ^^^  the three variables that might change during each turn of recursion
    // 2) exit conditions
    // 3) actually still (1), how to break up into smaller (sub)problems
    let mut dp = vec![vec![vec![-1; n as usize]; m as usize]; 1 + max_move as usize];
    dfs(&mut dp, m, n, max_move, start_row, start_column)
}

fn dfs(dp: &mut [Vec<Vec<i32>>], m: i32, n: i32, moves: i32, row: i32, col: i32) -> i32 {
    if moves < 0 {
        return 0;
    }
    if !(0..m).contains(&row) || !(0..n).contains(&col) {
        return 1;
    }
    if dp[moves as usize][row as usize][col as usize] > -1 {
        return dp[moves as usize][row as usize][col as usize];
    }
    let res = [
        dfs(dp, m, n, moves - 1, row - 1, col),
        dfs(dp, m, n, moves - 1, row + 1, col),
        dfs(dp, m, n, moves - 1, row, col - 1),
        dfs(dp, m, n, moves - 1, row, col + 1),
    ]
    .into_iter()
    .fold(0, |acc, n| (acc + n) % MOD);
    dp[moves as usize][row as usize][col as usize] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_paths(2, 2, 2, 0, 0), 6);
        debug_assert_eq!(find_paths(1, 3, 3, 0, 1), 12);
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
