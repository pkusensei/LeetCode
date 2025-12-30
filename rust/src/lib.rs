mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    if rows < 3 || cols < 3 {
        return 0;
    }
    let mut res = 0;
    for r in 0..=rows - 3 {
        for c in 0..=cols - 3 {
            if check(&grid, r, c) {
                res += 1;
            }
        }
    }
    res
}

fn check(grid: &[Vec<i32>], row: usize, col: usize) -> bool {
    let mut freq = [0; 9];
    for r in &grid[row..row + 3] {
        for &v in &r[col..col + 3] {
            if !(1..=9).contains(&v) {
                return false;
            }
            freq[v as usize - 1] += 1;
        }
    }
    if freq.into_iter().any(|n| n != 1) {
        return false;
    }
    const SUM: i32 = 15;
    grid[row][col..col + 3].iter().sum::<i32>() == SUM
        && grid[row + 1][col..col + 3].iter().sum::<i32>() == SUM
        && grid[row + 2][col..col + 3].iter().sum::<i32>() == SUM
        && grid[row][col] + grid[row + 1][col] + grid[row + 2][col] == SUM
        && grid[row][col + 1] + grid[row + 1][col + 1] + grid[row + 2][col + 1] == SUM
        && grid[row][col + 2] + grid[row + 1][col + 2] + grid[row + 2][col + 2] == SUM
        && grid[row][col] + grid[row + 1][col + 1] + grid[row + 2][col + 2] == SUM
        && grid[row][col + 2] + grid[row + 1][col + 1] + grid[row + 2][col] == SUM
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
