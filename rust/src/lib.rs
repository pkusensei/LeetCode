mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_ball(grid: &[&[i32]]) -> Vec<i32> {
    let cols = grid[0].len();
    (0..cols).map(|col| dfs(grid, 0, col)).collect()
}

fn dfs(grid: &[&[i32]], row: usize, col: usize) -> i32 {
    let rows = grid.len();
    if row == rows {
        return col as _;
    }
    if grid[row][col] == 1 && grid[row].get(1 + col).is_some_and(|&v| v == 1) {
        dfs(grid, 1 + row, 1 + col)
    } else if grid[row][col] == -1 && col.checked_sub(1).is_some_and(|c| grid[row][c] == -1) {
        dfs(grid, 1 + row, col - 1)
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            find_ball(&[
                &[1, 1, 1, -1, -1],
                &[1, 1, 1, -1, -1],
                &[-1, -1, -1, 1, 1],
                &[1, 1, 1, 1, -1],
                &[-1, -1, -1, -1, -1]
            ]),
            [1, -1, -1, -1, -1]
        );
        assert_eq!(find_ball(&[&[-1]]), [-1]);
        assert_eq!(
            find_ball(&[
                &[1, 1, 1, 1, 1, 1],
                &[-1, -1, -1, -1, -1, -1],
                &[1, 1, 1, 1, 1, 1],
                &[-1, -1, -1, -1, -1, -1]
            ]),
            [0, 1, 2, 3, 4, -1]
        );
    }

    #[test]
    fn test() {}
}
