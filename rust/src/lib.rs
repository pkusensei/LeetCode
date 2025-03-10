mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn has_valid_path(grid: &[&[char]]) -> bool {
    dfs(grid, 0, 0, 0, &mut HashMap::new())
}

fn dfs(
    grid: &[&[char]],
    r: usize,
    c: usize,
    mut open: i32,
    memo: &mut HashMap<(usize, usize, i32), bool>,
) -> bool {
    let [rows, cols] = get_dimensions(grid);
    if (rows + cols - 1) & 1 == 1 {
        return false; // path must has even length
    }
    let key = (r, c, open);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    open += if grid[r][c] == '(' { 1 } else { -1 };
    if open < 0 {
        return false;
    }
    if r == rows - 1 && c == cols - 1 {
        return open == 0;
    }
    let res = (1 + r < rows && dfs(grid, 1 + r, c, open, memo))
        || (1 + c < cols && dfs(grid, r, 1 + c, open, memo));
    memo.insert(key, res);
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
        assert!(has_valid_path(&[
            &['(', '(', '('],
            &[')', '(', ')'],
            &['(', '(', ')'],
            &['(', '(', ')']
        ]));
        assert!(!has_valid_path(&[&[')', ')'], &['(', '(']]))
    }

    #[test]
    fn test() {}
}
