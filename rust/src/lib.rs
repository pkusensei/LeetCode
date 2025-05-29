mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_minimum_time(strength: &[i32], k: i32) -> i32 {
    let n = strength.len();
    let mut memo = vec![vec![-1; 1 << n]; 2 + n];
    dfs(strength, k, 1, 0, &mut memo)
}

fn dfs(nums: &[i32], k: i32, x: i32, mask: usize, memo: &mut [Vec<i32>]) -> i32 {
    if nums.len() == mask.count_ones() as usize {
        return 0;
    }
    let depth = (x / k) as usize;
    if memo[depth][mask] > -1 {
        return memo[depth][mask];
    }
    let mut res = i32::MAX;
    for (i, &num) in nums.iter().enumerate() {
        if (mask >> i) & 1 == 0 {
            res = res.min((num + x - 1) / x + dfs(nums, k, x + k, mask | (1 << i), memo));
        }
    }
    memo[depth][mask] = res;
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
        assert_eq!(find_minimum_time(&[3, 4, 1], 1), 4);
    }

    #[test]
    fn test() {}
}
