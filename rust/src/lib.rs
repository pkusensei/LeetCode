mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum(nums: &[i32], k: i32, m: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let prefix = nums.iter().fold(vec![0], |mut acc, &num| {
        acc.push(num + acc.last().unwrap_or(&0));
        acc
    });
    let mut memo = vec![vec![[None; 2]; 1 + k]; n];
    dfs(&nums, &prefix, m as usize, 0, k, 0, &mut memo)
}

fn dfs(
    nums: &[i32],
    prefix: &[i32],
    m: usize,
    idx: usize,
    k: usize,
    insub: usize,
    memo: &mut [Vec<[Option<i32>; 2]>],
) -> i32 {
    let n = nums.len();
    if idx >= n {
        return if k == 0 { 0 } else { i32::MIN >> 1 };
    }
    if let Some(v) = memo[idx][k][insub] {
        return v;
    }
    let res = if insub == 1 {
        // End subarr vs Extend subarr
        dfs(nums, prefix, m, idx, k, 0, memo)
            .max(nums[idx] + dfs(nums, prefix, m, 1 + idx, k, insub, memo))
    } else {
        let mut res = dfs(nums, prefix, m, 1 + idx, k, insub, memo); // skip
        if idx + m <= n && 0 < k {
            res = res
                .max(prefix[idx + m] - prefix[idx] + dfs(nums, prefix, m, idx + m, k - 1, 1, memo));
        }
        res
    };
    memo[idx][k][insub] = Some(res);
    res
}

pub fn bottom_up(nums: &[i32], k: i32, m: i32) -> i32 {
    const INF: i32 = i32::MIN / 2;
    let n = nums.len();
    let [k, m] = [k, m].map(|v| v as usize);
    let prefix = nums.iter().fold(vec![0], |mut acc, &num| {
        acc.push(num + acc.last().unwrap_or(&0));
        acc
    });
    let mut dp = vec![0; 1 + n];
    let mk = m * k;
    for subarr_idx in 0..k {
        let mut curr_dp = vec![0; 1 + n];
        let min_pos_after_curr = m * (1 + subarr_idx);
        let min_pos_after_prev = m * subarr_idx;
        let mut best_prev = INF;
        for prev_end in m.max(min_pos_after_prev.saturating_sub(m))..min_pos_after_prev {
            best_prev = best_prev.max(dp[prev_end] - prefix[prev_end]);
        }
        curr_dp[..min_pos_after_curr].fill(INF);
        let mut best_curr = INF;
        let mut prev_idx = min_pos_after_prev;
        for curr_end in min_pos_after_curr..=n + min_pos_after_curr - mk {
            best_prev = best_prev.max(dp[prev_idx] - prefix[prev_idx]);
            prev_idx += 1;
            best_curr = best_curr.max(best_prev + prefix[curr_end]);
            curr_dp[curr_end] = best_curr
        }
        dp = curr_dp;
    }
    dp.into_iter().max().unwrap()
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
        assert_eq!(max_sum(&[1, 2, -1, 3, 3, 4], 2, 2), 13);
        assert_eq!(max_sum(&[-10, 3, -1, -2], 4, 1), -10);

        assert_eq!(bottom_up(&[1, 2, -1, 3, 3, 4], 2, 2), 13);
        assert_eq!(bottom_up(&[-10, 3, -1, -2], 4, 1), -10);
    }

    #[test]
    fn test() {}
}
