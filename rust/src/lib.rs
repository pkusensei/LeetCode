mod dsu;
mod helper;
mod trie;

use std::cmp::Reverse;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(mut nums: Vec<i32>, target: i32) -> i32 {
    let sum: i64 = nums.iter().map(|&v| i64::from(v)).sum();
    if sum < i64::from(target) {
        return -1;
    }
    nums.sort_unstable_by_key(|&v| Reverse(v));
    let mut res = 0;
    for bit in 0..31 {
        if (target >> bit) & 1 == 0 {
            continue;
        }
        let mut curr = 0;
        while curr < (1 << bit) {
            while nums.last().is_some_and(|&v| v <= (1 << bit)) && curr < (1 << bit) {
                let top = nums.pop().unwrap();
                curr += top;
            }
            if curr < (1 << bit) {
                res += 1;
                let top = nums.pop().unwrap();
                nums.extend([top / 2; 2]);
            }
        }
    }
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
        assert_eq!(min_operations(vec![1, 2, 8], 7), 1);
        assert_eq!(min_operations(vec![1, 32, 1, 2], 12), 2);
        assert_eq!(min_operations(vec![1, 32, 1], 35), -1);
    }

    #[test]
    fn test() {
        assert_eq!(min_operations(vec![1, 1, 1], 3), 0);
    }
}
