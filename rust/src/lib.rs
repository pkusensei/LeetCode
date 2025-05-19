mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let [mut memo1, mut memo2] = [0, 1].map(|_| vec![vec![[None; 128]; 1 + k]; n]);
    dfs(&nums, k, 0, 0, 0, &mut memo1);
    nums.reverse();
    dfs(&nums, k, 0, 0, 0, &mut memo2);
    let mut res = 0;
    for idx in k..=n - k {
        for or1 in 0..128 {
            for or2 in 0..128 {
                if let Some((true, true)) = memo1[idx][k][or1].zip(memo2[n - idx][k][or2]) {
                    res = res.max(or1 ^ or2);
                }
            }
        }
    }
    res as i32
}

fn dfs(
    nums: &[i32],
    k: usize,
    idx: usize,
    len: usize,
    orv: usize,
    memo: &mut [Vec<[Option<bool>; 128]>],
) -> bool {
    if idx >= nums.len() {
        return len == k;
    }
    if let Some(v) = memo[idx][len][orv] {
        return v;
    }
    let mut res = dfs(nums, k, 1 + idx, len, orv, memo); // skip
    if len < k {
        res |= dfs(nums, k, 1 + idx, 1 + len, orv | nums[idx] as usize, memo);
    }
    memo[idx][len][orv] = Some(res);
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
        assert_eq!(max_value(vec![2, 6, 7], 1), 5);
        assert_eq!(max_value(vec![4, 2, 5, 6, 7], 2), 2);
    }

    #[test]
    fn test() {}
}
