mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn min_abs_difference(nums: &[i32], goal: i32) -> i32 {
    let n = nums.len();
    let [mut left, mut right] = [0, 1].map(|_| HashSet::new());
    find_sums(&nums[..n / 2], 0, &mut left);
    find_sums(&nums[n / 2..], 0, &mut right);
    let mut right: Vec<_> = right.into_iter().collect();
    right.sort_unstable();
    let mut res = u32::MAX;
    for num in left {
        let target = goal - num;
        let i = right.partition_point(|&v| v < target);
        if let Some(&v) = right.get(i) {
            res = res.min((num + v).abs_diff(goal));
        }
        if let Some(_i) = i.checked_sub(1) {
            res = res.min((num + right[_i]).abs_diff(goal));
        }
    }
    res as _
}

fn find_sums(nums: &[i32], curr: i32, sums: &mut HashSet<i32>) {
    match nums {
        [] => {
            sums.insert(curr);
        }
        [head, tail @ ..] => {
            find_sums(tail, curr, sums);
            find_sums(tail, curr + head, sums);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(min_abs_difference(&[5, -7, 3, 5], 6), 0);
        assert_eq!(min_abs_difference(&[7, -9, 15, -2], -5), 1);
        assert_eq!(min_abs_difference(&[1, 2, 3], -7), 7);
    }

    #[test]
    fn test() {}
}
