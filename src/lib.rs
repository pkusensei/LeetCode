pub fn unique_paths_with_obstacles(obstacle_grid: &[&[i32]]) -> i32 {
    let row = obstacle_grid.len();
    let col = obstacle_grid.first().map(|r| r.len()).unwrap_or_default();
    if row * col == 0 || obstacle_grid[0][0] == 1 || obstacle_grid[row - 1][col - 1] == 1 {
        return 0;
    }

    let mut dp = vec![vec![0; col]; row];
    dp[0][0] = 1;
    for i in 0..row {
        for j in 0..col {
            if obstacle_grid[i][j] == 1 {
                dp[i][j] = 0;
            } else {
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }
    }
    dp[row - 1][col - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            unique_paths_with_obstacles(&[&[0, 0, 0], &[0, 1, 0], &[0, 0, 0]]),
            2
        );
        debug_assert_eq!(unique_paths_with_obstacles(&[&[0, 1], &[0, 0]]), 1);
    }

    #[test]
    fn test() {}
}
