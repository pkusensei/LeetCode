mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_dp(zero: i32, one: i32, limit: i32) -> i32 {
    let [num_zero, num_one, limit] = [zero, one, limit].map(|v| v as usize);
    // dp0[x][y]: count of arrays using x zeros and y ones
    let mut dp0 = vec![vec![0; 1 + num_one]; 1 + num_zero];
    let mut dp1 = vec![vec![0; 1 + num_one]; 1 + num_zero];
    // Base case: prefix streak of zeros
    for i in 1..=num_zero.min(limit) {
        dp0[i][0] = 1;
    }
    // Base case: prefix streak of ones
    for i in 1..=num_one.min(limit) {
        dp1[0][i] = 1;
    }
    for zero in 1..=num_zero {
        for one in 1..=num_one {
            // Append 0 to ..xx0 or ..xx1
            dp0[zero][one] = (dp0[zero - 1][one] + dp1[zero - 1][one]) % M;
            // Append 1 to ..xx0 or ..xx1
            dp1[zero][one] = (dp0[zero][one - 1] + dp1[zero][one - 1]) % M;
            if zero > limit {
                // dp1[x][y] always ends in 1,
                // dp1[zero-1-limit][one] has to add (1+limit) zeros to reach dp0[zero][one]
                dp0[zero][one] = (dp0[zero][one] - dp1[zero - 1 - limit][one]).rem_euclid(M);
            }
            if one > limit {
                dp1[zero][one] = (dp1[zero][one] - dp0[zero][one - 1 - limit]).rem_euclid(M);
            }
        }
    }
    (dp0[num_zero][num_one] + dp1[num_zero][num_one]) % M
}

pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    let [num_zero, num_one, limit] = [zero, one, limit].map(|v| v as usize);
    let mut memo = vec![vec![vec![vec![-1; 1 + limit]; 3]; 1 + num_one]; 1 + num_zero];
    dfs(num_zero, num_one, 2, 0, limit, &mut memo)
}

const M: i32 = 1_000_000_007;

fn dfs(
    num_zero: usize,
    num_one: usize,
    prev: usize,
    streak: usize,
    limit: usize,
    memo: &mut [Vec<Vec<Vec<i32>>>],
) -> i32 {
    if num_zero == 0 && num_one == 0 {
        return i32::from(streak <= limit);
    }
    if streak > limit {
        return 0;
    }
    if memo[num_zero][num_one][prev][streak] > -1 {
        return memo[num_zero][num_one][prev][streak];
    }
    let mut res = 0;
    if num_zero > 0 {
        res += dfs(
            num_zero - 1,
            num_one,
            0,
            if prev == 0 { 1 + streak } else { 1 },
            limit,
            memo,
        );
    }
    if num_one > 0 {
        res += dfs(
            num_zero,
            num_one - 1,
            1,
            if prev == 1 { 1 + streak } else { 1 },
            limit,
            memo,
        );
    }
    res %= M;
    memo[num_zero][num_one][prev][streak] = res;
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
