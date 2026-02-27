mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::Itertools;

pub fn at_most_n_given_digit_set(digits: &[&str], n: i32) -> i32 {
    let digits = digits
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();
    let n = n
        .to_string()
        .bytes()
        .map(|b| i32::from(b - b'0'))
        .collect_vec();
    let len = n.len();
    dfs(&digits, &n, 0, 1, 1, &mut vec![[[-1; 2]; 2]; len])
}

fn dfs(
    digits: &[i32],
    n: &[i32],
    idx: usize,
    tight: usize,
    leading: usize,
    memo: &mut [[[i32; 2]; 2]],
) -> i32 {
    if idx >= n.len() {
        return i32::from(leading == 0);
    }
    if memo[idx][tight][leading] > -1 {
        return memo[idx][tight][leading];
    }
    let upper = if tight == 1 { n[idx] } else { 9 };
    let mut res = 0;
    for d in 0..=upper {
        let ntight = tight & usize::from(d == n[idx]);
        if leading == 1 && d == 0 {
            res += dfs(digits, n, 1 + idx, ntight, leading, memo);
        } else if digits.binary_search(&d).is_ok() {
            res += dfs(digits, n, 1 + idx, ntight, 0, memo);
        }
    }
    memo[idx][tight][leading] = res;
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
        assert_eq!(at_most_n_given_digit_set(&["1", "3", "5", "7"], 100), 20);
    }

    #[test]
    fn test() {}
}
