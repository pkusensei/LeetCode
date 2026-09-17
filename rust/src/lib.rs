mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut queue = VecDeque::from([(0, 0, false, 0)]);
    let mut seen = vec![vec![[false; 2]; n]; n];
    seen[0][0][0] = true;
    while let Some((row, col, dir, step)) = queue.pop_front() {
        if row == n - 1 && col == n - 2 && !dir {
            return step;
        }
        if dir {
            if 2 + row < n && grid[2 + row][col] == 0 && !seen[1 + row][col][usize::from(dir)] {
                seen[1 + row][col][usize::from(dir)] = true;
                queue.push_back((1 + row, col, dir, 1 + step)); // move down
            }
            if 1 + col < n && grid[row][1 + col] == 0 && grid[1 + row][1 + col] == 0 {
                if !seen[row][col][0] {
                    seen[row][col][0] = true;
                    queue.push_back((row, col, false, 1 + step)); // rotate
                }
                if !seen[row][1 + col][usize::from(dir)] {
                    seen[row][1 + col][usize::from(dir)] = true;
                    queue.push_back((row, 1 + col, dir, 1 + step)); // move right
                }
            }
        } else {
            if grid[row].get(2 + col).is_some_and(|&v| v == 0)
                && !seen[row][2 + col][usize::from(dir)]
            {
                seen[row][2 + col][usize::from(dir)] = true;
                queue.push_back((row, 1 + col, dir, 1 + step)); // move right
            }
            if 1 + row < n && grid[1 + row][col] == 0 && grid[1 + row][1 + col] == 0 {
                if !seen[row][col][1] {
                    seen[row][col][1] = true;
                    queue.push_back((row, col, true, 1 + step)); // rotate
                }
                if !seen[1 + row][col][usize::from(dir)] {
                    seen[1 + row][col][usize::from(dir)] = true;
                    queue.push_back((1 + row, col, dir, 1 + step)); // move down
                }
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
