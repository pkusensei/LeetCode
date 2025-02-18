mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums.len();
    let seen: HashSet<i32> = nums
        .into_iter()
        .map(|s| i32::from_str_radix(&s, 2).unwrap_or_default())
        .collect();
    backtrack(n, 0, &seen)
        .map(|v| format!("{:0n$b}", v))
        .unwrap()
}

fn backtrack(n: usize, curr: i32, seen: &HashSet<i32>) -> Option<i32> {
    if n == 0 {
        return if !seen.contains(&curr) {
            Some(curr)
        } else {
            None
        };
    }
    backtrack(n - 1, curr << 1, seen).or_else(|| backtrack(n - 1, (curr << 1) | 1, seen))
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
