mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        dfs(&digits, 0, 0, &mut seen);
        seen.len() as i32
}

fn dfs(digits: &[i32], mask: i32, curr: i32, seen: &mut HashSet<i32>) {
    if mask.count_ones() == 3 {
        if curr & 1 == 0 {
            seen.insert(curr);
        }
        return;
    }
    for (i, &d) in digits.iter().enumerate() {
        if (mask >> i) & 1 == 0 && (d > 0 || mask > 0) {
            dfs(digits, mask | (1 << i), 10 * curr + d, seen);
        }
    }
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
