mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_space_wasted_k_resizing(nums: &[i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    dfs(nums, 0, k, &mut vec![vec![-1; n]; 1 + k])
}

fn dfs(nums: &[i32], idx: usize, k: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = nums.len();
    if idx >= n {
        return 0;
    }
    if memo[k][idx] > -1 {
        return memo[k][idx];
    }
    let mut res = i32::MAX;
    let mut max = 0;
    let mut window_sum = 0;
    for (next, &num) in nums[idx..].iter().enumerate() {
        max = max.max(num);
        window_sum += num;
        let curr = max * (next + 1) as i32 - window_sum;
        if k > 0 {
            res = res.min(curr + dfs(nums, idx + next + 1, k - 1, memo));
        }
    }
    if k == 0 {
        res = max * (n - idx) as i32 - window_sum;
    }
    memo[k][idx] = res;
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
        assert_eq!(min_space_wasted_k_resizing(&[10, 20], 0), 10);
        assert_eq!(min_space_wasted_k_resizing(&[10, 20, 30], 1), 10);
        assert_eq!(min_space_wasted_k_resizing(&[10, 20, 15, 30, 20], 2), 15);
    }

    #[test]
    fn test() {}
}
