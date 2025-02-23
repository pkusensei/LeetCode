mod dsu;
mod helper;
mod trie;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_difference(nums: &[i32]) -> i32 {
    let n = nums.len() / 2;
    let sum: i32 = nums.iter().sum();
    let [mut left, mut right] = [0, 1].map(|_| vec![BTreeSet::new(); 1 + n]);
    dfs(&nums[..n], 0, 0, &mut left);
    dfs(&nums[n..], 0, 0, &mut right);
    let mut res = i32::MAX;
    for len in 0..=n {
        for v1 in left[len].iter() {
            let target = sum / 2 - v1;
            if let Some(v2) = right[n - len].range(target..).next() {
                res = res.min((sum - 2 * (v1 + v2)).abs());
            }
            if let Some(v2) = right[n - len].range(..target).next_back() {
                res = res.min((sum - 2 * (v1 + v2)).abs());
            }
        }
    }
    res
}

fn dfs(nums: &[i32], len: usize, curr: i32, sums: &mut [BTreeSet<i32>]) {
    match nums {
        [] => {
            sums[len].insert(curr);
        }
        [head, tail @ ..] => {
            dfs(tail, len, curr, sums);
            dfs(tail, 1 + len, curr + *head, sums);
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
        assert_eq!(minimum_difference(&[-36, 36]), 72);
        assert_eq!(minimum_difference(&[3, 9, 7, 3]), 2);
        assert_eq!(minimum_difference(&[2, -1, 0, 4, -2, -9]), 0);
    }

    #[test]
    fn test() {}
}
