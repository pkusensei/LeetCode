pub fn num_magic_squares_inside(grid: &[&[i32]]) -> i32 {
    let row = grid.len();
    let col = grid.first().map(|r| r.len()).unwrap_or_default();
    if row < 3 || col < 3 {
        return 0;
    }
    let mut res = 0;
    for r in 0..=row - 3 {
        for c in 0..=col - 3 {
            if check(grid, r, c) {
                res += 1;
            }
        }
    }
    res
}

fn check(grid: &[&[i32]], row: usize, col: usize) -> bool {
    let mut nums = [0; 9];
    for r in row..row + 3 {
        for c in col..col + 3 {
            if !(1..=9).contains(&grid[r][c]) {
                return false;
            }
            nums[grid[r][c] as usize - 1] += 1;
        }
    }
    if !nums.into_iter().all(|n| n == 1) {
        return false;
    }

    let sum: i32 = grid[row][col..col + 3].iter().sum();

    grid[row + 1][col..col + 3].iter().sum::<i32>() == sum
        && grid[row + 2][col..col + 3].iter().sum::<i32>() == sum
        && grid[row][col] + grid[row + 1][col] + grid[row + 2][col] == sum
        && grid[row][col + 1] + grid[row + 1][col + 1] + grid[row + 2][col + 1] == sum
        && grid[row][col + 2] + grid[row + 1][col + 2] + grid[row + 2][col + 2] == sum
        && grid[row][col] + grid[row + 1][col + 1] + grid[row + 2][col + 2] == sum
        && grid[row][col + 2] + grid[row + 1][col + 1] + grid[row + 2][col] == sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            num_magic_squares_inside(&[&[4, 3, 8, 4], &[9, 5, 1, 9], &[2, 7, 6, 2]]),
            1
        );
        debug_assert_eq!(num_magic_squares_inside(&[&[8]]), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            num_magic_squares_inside(&[&[5, 5, 5], &[5, 5, 5], &[5, 5, 5]]),
            0
        );
        debug_assert_eq!(
            num_magic_squares_inside(&[&[10, 3, 5], &[1, 6, 11], &[7, 9, 2]]),
            0
        );
    }
}
