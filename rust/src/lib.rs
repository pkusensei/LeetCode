mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn total_numbers(digits: Vec<i32>) -> i32 {
    let mut freq = digits.iter().fold([0; 10], |mut acc, &v| {
        acc[v as usize] += 1;
        acc
    });
    let mut seen = HashSet::new();
    dfs(&mut freq, 0, 0, &mut seen);
    seen.len() as i32
}

fn dfs(freq: &mut [i32; 10], curr: i32, len: i32, seen: &mut HashSet<i32>) {
    if len == 3 {
        if curr >= 100 && (curr & 1) == 0 {
            seen.insert(curr);
        }
        return;
    }
    for i in 0..10 {
        if freq[i] > 0 {
            freq[i] -= 1;
            dfs(freq, curr * 10 + i as i32, 1 + len, seen);
            freq[i] += 1;
        }
    }
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
