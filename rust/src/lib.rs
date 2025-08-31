mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_difference(n: i32, k: i32) -> Vec<i32> {
    use itertools::Itertools;
    use std::collections::HashSet;
    let mut nums = HashSet::new();
    for p in 1..=n / 2 {
        if n % p == 0 {
            nums.extend([p, n / p]);
        }
    }
    let nums = nums.into_iter().sorted_unstable().collect_vec();
    let mut res = vec![];
    backtrack(n, &nums, k, &mut vec![], &mut res);
    res
}

fn backtrack(target: i32, nums: &[i32], k: i32, curr: &mut Vec<i32>, res: &mut Vec<i32>) {
    if 1 == target && 0 == k {
        let curr_diff = curr
            .last()
            .zip(curr.first())
            .map(|(b, a)| b - a)
            .unwrap_or(i32::MAX);
        let diff = res
            .last()
            .zip(res.first())
            .map(|(b, a)| b - a)
            .unwrap_or(i32::MAX);
        if curr_diff < diff {
            *res = curr.clone();
        }
        return;
    }
    if 1 >= target || k <= 0 || nums.is_empty() {
        return;
    }
    backtrack(target, &nums[1..], k, curr, res);
    if target % nums[0] == 0 {
        curr.push(nums[0]);
        backtrack(target / nums[0], nums, k - 1, curr, res);
        curr.pop();
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
    fn basics() {
        assert_eq!(min_difference(44, 3), [2, 2, 11]);
        assert_eq!(min_difference(100, 2), [10, 10]);
    }

    #[test]
    fn test() {
        assert_eq!(min_difference(4, 2), [2, 2]);
    }
}
