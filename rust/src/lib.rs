mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ways(pizza: &[&str], k: i32) -> i32 {
    let grid: Vec<_> = pizza.iter().map(|v| v.as_bytes()).collect();
    let [rows, cols] = get_dimensions(&grid);
    let k = k as usize;
    // dfs(
    //     &grid,
    //     [0, 0],
    //     k,
    //     &mut vec![vec![vec![-1; 1 + k]; cols]; rows],
    // )
    let mut apples = vec![vec![0; 1 + cols]; 1 + rows];
    for r in (0..rows).rev() {
        for c in (0..cols).rev() {
            apples[r][c] = i32::from(grid[r][c] == b'A') + apples[1 + r][c] + apples[r][1 + c]
                - apples[1 + r][1 + c];
        }
    }
    let mut dp = vec![vec![vec![0; 1 + k]; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            if apples[r][c] > 0 {
                dp[r][c][1] = 1;
            }
        }
    }
    for pieces in 2..=k {
        for r in 0..rows {
            for c in 0..cols {
                for cut in 1 + r..rows {
                    if apples[r][c] - apples[cut][c] > 0 {
                        dp[r][c][pieces] += dp[cut][c][pieces - 1];
                        dp[r][c][pieces] %= MOD;
                    }
                }
                for cut in 1 + c..cols {
                    if apples[r][c] - apples[r][cut] > 0 {
                        dp[r][c][pieces] += dp[r][cut][pieces - 1];
                        dp[r][c][pieces] %= MOD;
                    }
                }
            }
        }
    }
    dp[0][0][k]
}

const MOD: i32 = 1_000_000_007;

fn dfs(grid: &[&[u8]], [row, col]: [usize; 2], k: usize, memo: &mut [Vec<Vec<i32>>]) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    if k <= 1 {
        for r in row..rows {
            for c in col..cols {
                if grid[r][c] == b'A' {
                    return 1;
                }
            }
        }
        return 0;
    }
    if memo[row][col][k] > -1 {
        return memo[row][col][k];
    }
    let mut res = 0;
    let mut valid = false;
    'outer: for cut in 1 + row..rows {
        if valid {
            res += dfs(grid, [cut, col], k - 1, memo);
            res %= MOD;
            continue;
        }
        for r in row..cut {
            for c in col..cols {
                if grid[r][c] == b'A' {
                    valid = true;
                    res += dfs(grid, [cut, col], k - 1, memo);
                    res %= MOD;
                    continue 'outer;
                }
            }
        }
    }
    valid = false;
    'outer: for cut in 1 + col..cols {
        if valid {
            res += dfs(grid, [row, cut], k - 1, memo);
            res %= MOD;
            continue;
        }
        for r in row..rows {
            for c in col..cut {
                if grid[r][c] == b'A' {
                    valid = true;
                    res += dfs(grid, [row, cut], k - 1, memo);
                    res %= MOD;
                    continue 'outer;
                }
            }
        }
    }
    memo[row][col][k] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(ways(&["A..", "AAA", "..."], 3), 3);
        assert_eq!(ways(&["A..", "AA.", "..."], 3), 1);
        assert_eq!(ways(&["A..", "A..", "..."], 1), 1);
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
