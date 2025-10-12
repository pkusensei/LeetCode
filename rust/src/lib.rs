mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_balanced(s: String) -> i32 {
    use itertools::Itertools;
    let (s, n) = (s.as_bytes(), s.len());
    let mut res = 1;
    for left in 0..n {
        let mut f = [0; 26];
        for right in left..n {
            f[usize::from(s[right] - b'a')] += 1;
            if f.into_iter().filter(|&v| v > 0).all_equal() {
                res = res.max(right + 1 - left);
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
