mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn calculate_minimum_hp(grid: &[&[i32]]) -> i32 {
    let (row, col) = (grid.len(), grid.first().map(|r| r.len()).unwrap_or(0));
    if row * col < 1 {
        return 0;
    }

    let mut dp = vec![vec![1; col]; row];
    dp[row - 1][col - 1] = 1.max(1 - grid[row - 1][col - 1]);

    for (j, &num) in grid[row - 1].iter().enumerate().rev() {
        if j == col - 1 {
            continue;
        }
        dp[row - 1][j] = 1.max(dp[row - 1][j + 1] - num);
    }

    for i in (0..row - 1).rev() {
        dp[i][col - 1] = 1.max(dp[i + 1][col - 1] - grid[i][col - 1]);
    }

    for i in (0..row - 1).rev() {
        for j in (0..col - 1).rev() {
            let right = 1.max(dp[i][j + 1] - grid[i][j]);
            let down = 1.max(dp[i + 1][j] - grid[i][j]);
            dp[i][j] = right.min(down);
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            calculate_minimum_hp(&[&[-2, -3, 3], &[-5, -10, 1], &[10, 30, -5]]),
            7
        );
        debug_assert_eq!(calculate_minimum_hp(&[&[0]]), 1);
    }

    #[test]
    fn test() {}
}
