mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut memo = vec![[[-1; 201]; 201]; n];
    dfs(&nums, 0, 0, 0, &mut memo)
}

pub fn bottom_up(nums: &[i32]) -> i32 {
    const fn gcd(a: usize, b: usize) -> usize {
        if a == 0 { return b } else { gcd(b % a, a) }
    }

    let n = nums.len();
    let mut dp = [[0; 201]; 201];
    dp[0][0] = 1;
    for &num in nums.iter() {
        let mut curr = [[0; 201]; 201];
        let num = num as usize;
        for a in 0..201 {
            let gcd1 = gcd(a, num);
            for b in 0..201 {
                if dp[a][b] == 0 {
                    continue;
                }
                let gcd2 = gcd(b, num);
                curr[a][b] = (curr[a][b] + dp[a][b]) % M;
                curr[gcd1][b] = (curr[gcd1][b] + dp[a][b]) % M;
                curr[a][gcd2] = (curr[a][gcd2] + dp[a][b]) % M;
            }
        }
        dp = curr;
    }
    (1..201).map(|i| dp[i][i]).fold(0, |acc, v| (acc + v) % M)
}

const M: i32 = 1_000_000_007;
fn dfs(nums: &[i32], idx: usize, gcd1: i32, gcd2: i32, memo: &mut [[[i32; 201]; 201]]) -> i32 {
    const fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 { return b } else { gcd(b % a, a) }
    }

    if idx >= nums.len() {
        return i32::from(gcd1 == gcd2 && gcd1 > 0);
    }
    if memo[idx][gcd1 as usize][gcd2 as usize] > -1 {
        return memo[idx][gcd1 as usize][gcd2 as usize];
    }
    let skip = dfs(nums, 1 + idx, gcd1, gcd2, memo);
    let take1 = dfs(nums, 1 + idx, gcd(gcd1, nums[idx]), gcd2, memo);
    let take2 = dfs(nums, 1 + idx, gcd1, gcd(gcd2, nums[idx]), memo);
    let res = ((skip + take1) % M + take2) % M;
    memo[idx][gcd1 as usize][gcd2 as usize] = res;
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
