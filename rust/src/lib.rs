mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    // dp[zero][one][0 or 1]
    let [zero, one, limit] = [zero, one, limit].map(|v| v as usize);
    let mut dp = vec![vec![[0_i64; 2]; 1 + one]; 1 + zero];
    for ze in 0..=zero.min(limit) {
        dp[ze][0][0] = 1;
    }
    for on in 0..=one.min(limit) {
        dp[0][on][1] = 1;
    }
    for ze in 1..=zero {
        for on in 1..=one {
            dp[ze][on][0] = (dp[ze - 1][on][0] + dp[ze - 1][on][1]
                - ze.checked_sub(limit + 1)
                    .map(|prev| dp[prev][on][1])
                    .unwrap_or(0))
            .rem_euclid(M);
            dp[ze][on][1] = (dp[ze][on - 1][1] + dp[ze][on - 1][0]
                - on.checked_sub(1 + limit)
                    .map(|prev| dp[ze][prev][0])
                    .unwrap_or(0))
            .rem_euclid(M)
        }
    }
    ((dp[zero][one][0] + dp[zero][one][1]) % M) as i32
}

const M: i64 = 1_000_000_007;

pub fn top_down(zero: i32, one: i32, limit: i32) -> i32 {
    let mut memo =
        vec![vec![vec![vec![-1; 1 + limit as usize]; 3]; 1 + one as usize]; 1 + zero as usize];
    dfs(zero, one, limit, 2, 0, &mut memo) as i32
}

fn dfs(
    num_zero: i32,
    num_one: i32,
    limit: i32,
    prev: i32,
    count: i32,
    memo: &mut [Vec<Vec<Vec<i64>>>],
) -> i64 {
    if num_zero == 0 && num_one == 0 {
        return i64::from(count <= limit);
    }
    if num_zero < 0 || num_one < 0 || count > limit {
        return 0;
    }
    if memo[num_zero as usize][num_one as usize][prev as usize][count as usize] > -1 {
        return memo[num_zero as usize][num_one as usize][prev as usize][count as usize];
    }
    let res = dfs(
        num_zero - 1,
        num_one,
        limit,
        0,
        if prev == 0 { 1 + count } else { 1 },
        memo,
    ) + dfs(
        num_zero,
        num_one - 1,
        limit,
        1,
        if prev == 1 { 1 + count } else { 1 },
        memo,
    );
    memo[num_zero as usize][num_one as usize][prev as usize][count as usize] = res % M;
    memo[num_zero as usize][num_one as usize][prev as usize][count as usize]
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
        assert_eq!(number_of_stable_arrays(1, 1, 2), 2);
        assert_eq!(number_of_stable_arrays(1, 2, 1), 1);
        assert_eq!(number_of_stable_arrays(3, 3, 2), 14);

        assert_eq!(top_down(1, 1, 2), 2);
        assert_eq!(top_down(1, 2, 1), 1);
        assert_eq!(top_down(3, 3, 2), 14);
    }

    #[test]
    fn test() {}
}
