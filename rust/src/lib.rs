mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = [m, n].map(|v| v as usize);
    let mut grid = vec![vec![0; cols]; rows];
    for w in walls.iter().chain(&guards) {
        grid[w[0] as usize][w[1] as usize] = 3;
    }
    for g in guards {
        let [row, col] = [0, 1].map(|i| g[i] as usize);
        for r in 1 + row..rows {
            if grid[r][col] > 2 {
                break;
            }
            grid[r][col] += 1;
        }
        for r in (0..row).rev() {
            if grid[r][col] > 2 {
                break;
            }
            grid[r][col] += 1;
        }
        for c in 1 + col..cols {
            if grid[row][c] > 2 {
                break;
            }
            grid[row][c] += 1;
        }
        for c in (0..col).rev() {
            if grid[row][c] > 2 {
                break;
            }
            grid[row][c] += 1;
        }
    }
    grid.into_iter().flatten().filter(|&v| v == 0).count() as i32
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
