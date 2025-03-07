mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value_of_coins(piles: &[&[i32]], k: i32) -> i32 {
    let n = piles.len();
    let k = k as usize;
    let mut prefix = Vec::with_capacity(n);
    for pile in piles.iter() {
        let curr = pile.iter().take(k).fold(vec![], |mut acc, &num| {
            acc.push(num + acc.last().unwrap_or(&0));
            acc
        });
        prefix.push(curr);
    }
    dfs(&prefix, 0, k, &mut vec![vec![-1; 1 + k]; n])
}

fn dfs(prefix: &[Vec<i32>], idx: usize, k: usize, memo: &mut [Vec<i32>]) -> i32 {
    if k == 0 || idx >= prefix.len() {
        return 0;
    }
    if memo[idx][k] > -1 {
        return memo[idx][k];
    }
    let curr = &prefix[idx];
    let mut res = dfs(prefix, 1 + idx, k, memo);
    for i in 0..curr.len().min(k) {
        res = res.max(curr[i] + dfs(prefix, 1 + idx, k - i - 1, memo));
    }
    memo[idx][k] = res;
    res
}

pub fn tabulation(piles: &[&[i32]], k: i32) -> i32 {
    let k = k as usize;
    let mut dp = vec![0; 1 + k];
    for pile in piles {
        let prefix = pile.iter().fold(vec![], |mut acc, &num| {
            acc.push(num + acc.last().unwrap_or(&0));
            acc
        });
        for i in (1..=k).rev() {
            for curr in 1..=i.min(pile.len()) {
                dp[i] = dp[i].max(dp[i - curr] + prefix[curr - 1]);
            }
        }
    }
    dp[k]
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
        assert_eq!(max_value_of_coins(&[&[1, 100, 3], &[7, 8, 9]], 2), 101);
        assert_eq!(
            max_value_of_coins(
                &[
                    &[100],
                    &[100],
                    &[100],
                    &[100],
                    &[100],
                    &[100],
                    &[1, 1, 1, 1, 1, 1, 700]
                ],
                7
            ),
            706
        );

        assert_eq!(tabulation(&[&[1, 100, 3], &[7, 8, 9]], 2), 101);
        assert_eq!(
            tabulation(
                &[
                    &[100],
                    &[100],
                    &[100],
                    &[100],
                    &[100],
                    &[100],
                    &[1, 1, 1, 1, 1, 1, 700]
                ],
                7
            ),
            706
        );
    }

    #[test]
    fn test() {}
}
