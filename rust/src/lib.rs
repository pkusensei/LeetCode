mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(&grid);
    let layers = rows.min(cols) / 2;
    for lay in 0..layers {
        let mut curr = grid[lay][lay..cols - lay - 1].to_vec();
        for r in lay..rows - lay - 1 {
            curr.push(grid[r][cols - lay - 1]);
        }
        for c in (1 + lay..cols - lay).rev() {
            curr.push(grid[rows - lay - 1][c]);
        }
        for r in (1 + lay..rows - lay).rev() {
            curr.push(grid[r][lay]);
        }
        let k = k as usize % curr.len();
        curr.rotate_left(k);
        let mut idx = 0;
        for c in lay..cols - lay - 1 {
            grid[lay][c] = curr[idx];
            idx += 1;
        }
        for r in lay..(rows - lay - 1) {
            grid[r][cols - lay - 1] = curr[idx];
            idx += 1;
        }
        for c in (1 + lay..cols - lay).rev() {
            grid[rows - lay - 1][c] = curr[idx];
            idx += 1;
        }
        for r in (1 + lay..rows - lay).rev() {
            grid[r][lay] = curr[idx];
            idx += 1;
        }
    }
    grid
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
