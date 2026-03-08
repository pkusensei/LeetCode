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

pub fn with_cantor(nums: &[&str]) -> String {
    let res: Vec<_> = nums
        .iter()
        .enumerate()
        .map(|(i, s)| b'0' + u8::from(s.as_bytes()[i] == b'0'))
        .collect();
    String::from_utf8(res).unwrap()
}

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums.len();
    let seen: HashSet<_> = nums
        .iter()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();
    let v = backtrack(&seen, n, 0).unwrap_or_default();
    format!("{:0n$b}", v)
}

fn backtrack(seen: &HashSet<i32>, n: usize, curr: i32) -> Option<i32> {
    if n == 0 {
        return if seen.contains(&curr) {
            None
        } else {
            Some(curr)
        };
    }
    backtrack(seen, n - 1, curr << 1).or_else(|| backtrack(seen, n - 1, (curr << 1) | 1))
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
