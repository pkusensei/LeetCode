mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::chain;

pub fn longest_palindrome(s: String, t: String) -> i32 {
    let (s1, n1) = (s.as_bytes(), s.len());
    let (s2, n2) = (t.as_bytes(), t.len());
    let mut res = 1;
    for a in 0..n1 {
        for b in a..=n1 {
            for c in 0..n2 {
                for d in c..=n2 {
                    let it = chain!(&s1[a..b], &s2[c..d]);
                    if is_palindrome(it.clone()) {
                        res = res.max(it.count());
                    }
                }
            }
        }
    }
    res as i32
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
