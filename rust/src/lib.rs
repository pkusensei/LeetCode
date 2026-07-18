mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(source: &str, target: &str, rules: &[[&str; 2]], costs: &[i32]) -> i32 {
    use itertools::izip;
    let n = source.len();
    if n != target.len() {
        return -1;
    }
    if source == target {
        return 0;
    }
    let (s, t) = (source.as_bytes(), target.as_bytes());
    let mut dp = vec![i32::MAX >> 1; 1 + n];
    dp[0] = 0;
    for si in 0..n {
        if s[si] == t[si] {
            dp[1 + si] = dp[1 + si].min(dp[si]);
        }
        'out: for (rule, &cost) in izip!(rules.iter(), costs.iter()) {
            let len = rule[0].len();
            if si + len > n {
                continue;
            }
            let mut curr_cost = cost;
            for (bi, (pat, rep)) in izip!(rule[0].bytes(), rule[1].bytes()).enumerate() {
                if (pat == s[si + bi] || pat == b'*') && rep == t[si + bi] {
                    curr_cost += i32::from(pat == b'*');
                } else {
                    continue 'out;
                }
            }
            dp[si + len] = dp[si + len].min(curr_cost + dp[si]);
        }
    }
    if dp[n] >= i32::MAX >> 1 { -1 } else { dp[n] }
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
        assert_eq!(min_cost("test", "next", &[["*e*t", "next"]], &[4]), 6);
        assert_eq!(
            min_cost("hello", "world", &[["he", "wo"], ["llo", "rld"]], &[3, 4]),
            7
        );
        assert_eq!(min_cost("cat", "dog", &[["c*t", "dog"]], &[2]), 3);
        assert_eq!(min_cost("ab", "bc", &[["a*", "bd"]], &[9]), -1);
    }

    #[test]
    fn test() {}
}
