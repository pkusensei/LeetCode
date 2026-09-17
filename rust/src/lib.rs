mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn with_dp(arr: &[i32], target: i32) -> i32 {
    let n = arr.len();
    let mut dp = vec![None; n];
    let mut left = 0;
    let mut res = None;
    let mut min_len = None;
    let mut sum = 0;
    for (right, &num) in arr.iter().enumerate() {
        sum += num;
        while sum > target {
            sum -= arr[left];
            left += 1;
        }
        if sum == target {
            let curr = 1 + right - left;
            let v = min_len.get_or_insert(curr);
            *v = (*v).min(curr);
            if let Some(left) = left.checked_sub(1)
                && let Some(v) = dp[left]
            {
                let curr = (curr + v) as i32;
                let v = res.get_or_insert(curr);
                *v = (*v).min(curr);
            }
        }
        dp[right] = min_len;
    }
    res.unwrap_or(-1)
}

pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let n = arr.len();
    let mut prefix = vec![1 + n as i32; n];
    let mut map = HashMap::from([(0, -1)]);
    let mut sum = 0;
    for (right, &num) in arr.iter().enumerate() {
        sum += num;
        if let Some(prev) = map.get(&(sum - target)) {
            let curr = right as i32 - prev;
            prefix[right] = curr;
        }
        if right > 0 {
            prefix[right] = prefix[right].min(prefix[right - 1]);
        }
        map.insert(sum, right as i32);
    }
    map = HashMap::from([(0, n as i32)]);
    sum = 0;
    let mut suffix = vec![1 + n as i32; n];
    for (left, &num) in arr.iter().enumerate().rev() {
        sum += num;
        if let Some(prev) = map.get(&(sum - target)) {
            let curr = prev - left as i32;
            suffix[left] = curr;
        }
        if 1 + left < n {
            suffix[left] = suffix[left].min(suffix[1 + left]);
        }
        map.insert(sum, left as i32);
    }
    let mut res = None;
    for i in 0..n - 1 {
        if prefix[i] <= n as i32 && suffix[1 + i] <= n as i32 {
            let curr = prefix[i] + suffix[1 + i];
            let v = res.get_or_insert(curr);
            *v = (*v).min(curr);
        }
    }
    res.unwrap_or(-1)
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
