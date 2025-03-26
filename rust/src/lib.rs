mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_possible_to_cut_path(mut grid: Vec<Vec<i32>>) -> bool {
    let [rows, cols] = get_dimensions(&grid);
    if (rows == 1 && cols <= 2) || (rows <= 2 && cols == 1) {
        return false;
    }
    if dfs(&mut grid, 0, 0) {
        grid[0][0] = 1;
        if dfs(&mut grid, 0, 0) {
            return false;
        }
    }
    true
}

fn dfs(grid: &mut [Vec<i32>], r: usize, c: usize) -> bool {
    let [rows, cols] = get_dimensions(grid);
    if grid[r][c] == 0 {
        return false;
    }
    if r == rows - 1 && c == cols - 1 {
        return true;
    }
    grid[r][c] = 0;
    (1 + r < rows && dfs(grid, 1 + r, c)) || (1 + c < cols && dfs(grid, r, 1 + c))
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
    fn basics() {
        assert!(is_possible_to_cut_path(vec![
            vec![1, 1, 1],
            vec![1, 0, 0],
            vec![1, 1, 1]
        ]));
        assert!(!is_possible_to_cut_path(vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1]
        ]));
    }

    #[test]
    fn test() {
        assert!(is_possible_to_cut_path(vec![vec![1, 1, 1, 1, 1]]));
    }
}
