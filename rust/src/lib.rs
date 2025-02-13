mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(&grid);
    let layers = rows.min(cols) / 2;
    for layer in 0..layers {
        let mut curr = vec![];
        curr.extend_from_slice(&grid[layer][layer..cols - layer - 1]); // top
        for r in layer..rows - layer - 1 {
            curr.push(grid[r][cols - layer - 1]); // right
        }
        for c in (1 + layer..cols - layer).rev() {
            curr.push(grid[rows - layer - 1][c]); // bottom
        }
        for r in (1 + layer..rows - layer).rev() {
            curr.push(grid[r][layer]);
        }
        let v = k as usize % curr.len();
        curr.rotate_left(v);
        let mut idx = 0;
        for c in layer..cols - layer - 1 {
            grid[layer][c] = curr[idx];
            idx += 1;
        }
        for r in layer..rows - layer - 1 {
            grid[r][cols - layer - 1] = curr[idx];
            idx += 1;
        }
        for c in (1 + layer..cols - layer).rev() {
            grid[rows - layer - 1][c] = curr[idx];
            idx += 1;
        }
        for r in (1 + layer..rows - layer).rev() {
            grid[r][layer] = curr[idx];
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
    fn basics() {}

    #[test]
    fn test() {}
}
