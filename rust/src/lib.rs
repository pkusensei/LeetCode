mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
    use std::collections::VecDeque;
    let [rows, cols] = get_dimensions(&grid);
    let mut queue = VecDeque::from([(0, 0, health - grid[0][0])]);
    let mut seen = vec![vec![false; cols]; rows];
    while let Some((r, c, h)) = queue.pop_front() {
        if r == rows - 1 && c == cols - 1 {
            return h > 0;
        }
        for [nr, nc] in neighbors([r, c]) {
            if nr < rows && nc < cols && !seen[nr][nc] {
                seen[nr][nc] = true;
            }
            if grid[nr][nc] == 0 {
                queue.push_front((nr, nc, h));
            } else if h > 1 {
                queue.push_back((nr, nc, h - 1));
            }
        }
    }
    false
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
