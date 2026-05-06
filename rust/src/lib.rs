mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut start = [0, 0];
    let mut ban = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            match v {
                1 => start = [r, c],
                -1 => ban += 1,
                _ => (),
            }
        }
    }
    dfs(&mut grid, start[0], start[1], (rows * cols) as i32 - ban)
}

fn dfs(grid: &mut [Vec<i32>], row: usize, col: usize, count: i32) -> i32 {
    if grid[row][col] == 2 {
        return (count == 1).into();
    }
    let mut res = 0;
    grid[row][col] = -1;
    for [nr, nc] in neighbors([row, col]) {
        if grid
            .get(nr)
            .is_some_and(|r| r.get(nc).is_some_and(|&v| v >= 0))
        {
            res += dfs(grid, nr, nc, count - 1)
        }
    }
    grid[row][col] = 0;
    res
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
    fn basics() {
        assert_eq!(
            unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
            2
        );
    }

    #[test]
    fn test() {}
}
