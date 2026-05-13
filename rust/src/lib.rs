mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
    let n = nums.len();
    let limit = limit as usize;
    let mut diff = vec![0; 2 + 2 * limit];
    for i in 0..n / 2 {
        let a = nums[i].min(nums[n - i - 1]) as usize;
        let b = nums[i].max(nums[n - i - 1]) as usize;
        // change both `a` and `b` to 1
        diff[2] += 2;
        // 2 < sum <= a requires 2 changes
        // What happens if a == b == 1?
        // Removing the impact on `[1+a]` and `[a+b]` offsets earlier `2`
        diff[a + 1] -= 1;
        // 1+a <= sum < a+b requires 1 change
        // sum == a+b is no change
        diff[a + b] -= 1;
        // a+b < sum <= b+limit requires 1 change
        diff[a + b + 1] += 1;
        // b+limit < sum
        diff[b + limit + 1] += 1;
    }
    let mut res = n as i32;
    let mut prefix = 0;
    for sum in 2..=2 * limit {
        prefix += diff[sum];
        res = res.min(prefix);
    }
    res
}

pub fn with_binary_search(nums: Vec<i32>, limit: i32) -> i32 {
    use std::collections::HashMap;

    let n = nums.len();
    let mut sum_freq = HashMap::new();
    let mut mins = Vec::with_capacity(n / 2);
    let mut maxs = Vec::with_capacity(n / 2);
    for i in 0..n / 2 {
        let a = nums[i].min(nums[n - i - 1]);
        let b = nums[i].max(nums[n - i - 1]);
        *sum_freq.entry(a + b).or_insert(0) += 1;
        mins.push(a);
        maxs.push(b);
    }
    mins.sort_unstable();
    maxs.sort_unstable();
    let mut res = n;
    for sum in 2..=2 * limit {
        let add_left = n / 2 - mins.partition_point(|&v| v < sum);
        let add_right = maxs.partition_point(|&v| v < sum - limit);
        let curr = n / 2 + add_left + add_right - sum_freq.get(&sum).unwrap_or(&0);
        res = res.min(curr);
    }
    res as i32
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
