mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_palindromic_subsequence(s: &str, k: i32) -> i32 {
    let n = s.len();
    let k = k as usize;
    let mut memo = vec![vec![vec![-1; 1 + k]; n]; n];
    dfs(s.as_bytes(), 0, n - 1, k as _, &mut memo)
}

fn dfs(s: &[u8], left: usize, right: usize, k: usize, memo: &mut [Vec<Vec<i32>>]) -> i32 {
    if left == right {
        return 1;
    }
    if left > right {
        return 0;
    }
    if memo[left][right][k] > -1 {
        return memo[left][right][k];
    }
    let mut res = dfs(s, 1 + left, right - 1, k, memo)
        .max(dfs(s, 1 + left, right, k, memo))
        .max(dfs(s, left, right - 1, k, memo));
    let diff = {
        let d = s[left].abs_diff(s[right]);
        usize::from(d.min(26 - d))
    };
    if let Some(v) = k.checked_sub(diff) {
        res = res.max(2 + dfs(s, 1 + left, right - 1, v, memo));
    }
    memo[left][right][k] = res;
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
    fn basics() {
        assert_eq!(longest_palindromic_subsequence("abced", 2), 3);
        assert_eq!(longest_palindromic_subsequence("aaazzz", 4), 6);
    }

    #[test]
    fn test() {}
}
