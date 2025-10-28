mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_palindromic_subsequences(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut memo = vec![vec![-1_i32; n]; n];
    dfs(s, 0, n - 1, &mut memo)
}

const M: i32 = 1_000_000_007;

fn dfs(s: &[u8], left: usize, right: usize, memo: &mut [Vec<i32>]) -> i32 {
    if left > right {
        return 0;
    }
    if left == right {
        return 1;
    }
    if memo[left][right] > -1 {
        return memo[left][right];
    }
    let res = if s[left] == s[right] {
        let mut res = 2 * dfs(s, 1 + left, right - 1, memo) % M;
        let mut inner_left = 1 + left;
        let mut inner_right = right - 1;
        while inner_left <= inner_right && s[inner_left] != s[left] {
            inner_left += 1;
        }
        while inner_left <= inner_right && s[inner_right] != s[left] {
            inner_right -= 1;
        }
        if inner_left < inner_right {
            res -= dfs(s, 1 + inner_left, inner_right - 1, memo)
        } else if inner_left == inner_right {
            res += 1;
        } else {
            res += 2;
        }
        res.rem_euclid(M)
    } else {
        (dfs(s, 1 + left, right, memo) + dfs(s, left, right - 1, memo)
            - dfs(s, 1 + left, right - 1, memo))
        .rem_euclid(M)
    };
    memo[left][right] = res;
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
