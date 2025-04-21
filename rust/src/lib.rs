mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn missing_integer(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let n = nums.len();
    let mut prefix = nums[0];
    for i in 1..n {
        if nums[i] == 1 + nums[i - 1] {
            prefix += nums[i];
        } else {
            break;
        }
    }
    let set: HashSet<_> = nums.into_iter().collect();
    while set.contains(&prefix) {
        prefix += 1;
    }
    prefix
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
