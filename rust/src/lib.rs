mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_operations(nums: &[i32], start: i32, goal: i32) -> i32 {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::from([start]);
    while let Some((node, step)) = queue.pop_front() {
        if node == goal {
            return step;
        }
        if !(0..=1000).contains(&node) {
            continue;
        }
        for &num in nums.iter() {
            let a = num + node;
            if seen.insert(a) {
                queue.push_back((a, 1 + step));
            }
            let b = -num + node;
            if seen.insert(b) {
                queue.push_back((b, 1 + step));
            }
            let c = num ^ node;
            if seen.insert(c) {
                queue.push_back((c, 1 + step));
            }
        }
    }
    -1
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
