mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_balanced(low: i64, high: i64) -> i64 {
    digit_dp(high) - digit_dp(low - 1)
}

fn digit_dp(num: i64) -> i64 {
    let s: Vec<_> = num
        .to_string()
        .into_bytes()
        .into_iter()
        .map(|b| u16::from(b - b'0'))
        .collect();
    let n = s.len();
    let mut memo = vec![vec![vec![vec![-1; 9 * n]; 9 * n]; 2]; n];
    dfs(&s, 0, true, 0, 0, &mut memo)
}

fn dfs(
    s: &[u16],
    idx: usize,
    tight: bool,
    sum_odd: u16,
    sum_even: u16,
    memo: &mut [Vec<Vec<Vec<i64>>>],
) -> i64 {
    if idx >= s.len() {
        return i64::from(sum_odd == sum_even && sum_odd > 0);
    }
    if memo[idx][usize::from(tight)][usize::from(sum_odd)][usize::from(sum_even)] > -1 {
        return memo[idx][usize::from(tight)][usize::from(sum_odd)][usize::from(sum_even)];
    }
    let upper = if tight { s[idx] } else { 9 };
    let mut res = 0;
    for d in 0..=upper {
        let ntight = tight && d == upper;
        res += if idx & 1 == 1 {
            dfs(s, 1 + idx, ntight, d + sum_odd, sum_even, memo)
        } else {
            dfs(s, 1 + idx, ntight, sum_odd, d + sum_even, memo)
        };
    }
    memo[idx][usize::from(tight)][usize::from(sum_odd)][usize::from(sum_even)] = res;
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
        assert_eq!(count_balanced(1, 100), 9);
        assert_eq!(count_balanced(120, 129), 1);
    }

    #[test]
    fn test() {
        assert_eq!(
            count_balanced(810591751771967, 823082845685977),
            289395497430
        );
    }
}
