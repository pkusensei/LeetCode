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

pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
    let [rows, cols] = get_dimensions(&grid);
    let mut queue = VecDeque::from([(0, 0, health - grid[0][0])]);
    let mut seen = vec![vec![0; cols]; rows];
    seen[0][0] = health - grid[0][0];
    while let Some((r, c, h)) = queue.pop_front() {
        if r == rows - 1 && c == cols - 1 {
            break;
        }
        for [nr, nc] in neighbors([r, c]) {
            if nr < rows && nc < cols {
                let next = h - grid[nr][nc];
                if seen[nr][nc] < next {
                    seen[nr][nc] = next;
                    if next == h {
                        queue.push_front((nr, nc, next));
                    } else {
                        queue.push_back((nr, nc, next));
                    }
                }
            }
        }
    }
    seen[rows - 1][cols - 1] > 0
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
