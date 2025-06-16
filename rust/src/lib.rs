mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_palindrome(s: String, t: String) -> i32 {
    use itertools::{Itertools, chain};
    let s1 = s.into_bytes();
    let s2 = t.into_bytes().into_iter().rev().collect_vec();
    let pal1 = get_pals(&s1);
    let pal2 = get_pals(&s2);
    let (n1, n2) = (s1.len(), s2.len());
    let mut res = chain!(pal1.iter(), pal2.iter()).copied().max().unwrap_or(1);
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for i1 in 1..=n1 {
        for i2 in 1..=n2 {
            if s1[i1 - 1] == s2[i2 - 1] {
                dp[i1][i2] = dp[i1][i2].max(1 + dp[i1 - 1][i2 - 1]);
                res = res
                    .max(2 * dp[i1][i2] + pal1.get(i1).unwrap_or(&0))
                    .max(2 * dp[i1][i2] + pal2.get(i2).unwrap_or(&0));
            }
        }
    }
    res as i32
}

fn get_pals(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n];
    for i in 0..n {
        expand(s, i, i, &mut res);
        expand(s, i, 1 + i, &mut res);
    }
    res
}

fn expand(s: &[u8], mut left: usize, mut right: usize, pals: &mut [usize]) {
    while s.get(left).zip(s.get(right)).is_some_and(|(a, b)| a == b) {
        pals[left] = pals[left].max(right + 1 - left);
        left = left.wrapping_sub(1);
        right += 1;
    }
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
        assert_eq!(longest_palindrome("a".into(), "a".into()), 2);
        assert_eq!(longest_palindrome("abc".into(), "def".into()), 1);
        assert_eq!(longest_palindrome("b".into(), "aaaa".into()), 4);
        assert_eq!(longest_palindrome("abcde".into(), "ecdba".into()), 5);
    }

    #[test]
    fn test() {}
}
