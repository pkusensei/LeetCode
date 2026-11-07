mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_valid_substrings(s: &str) -> i64 {
    let n = s.len();
    let mut res = 0;
    let mut prev = 0;
    for i in 0..n {
        if 2 <= i && &s[i - 2..=i] == "abc" {
            prev = i - 1;
        }
        res += i - prev + 1;
    }
    res as i64
}

pub fn with_split(s: &str) -> i64 {
    let n = s.len();
    let mut res = 0;
    let mut len = 0;
    for i in 0..n {
        len += 1;
        if 2 <= i && &s[i - 2..=i] == "abc" {
            // double counted middle 'b'
            res += (len - 1) * len / 2 - 1;
            len = 2;
        }
    }
    res + len * (1 + len) / 2
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
    fn test() {
        assert_eq!(count_valid_substrings("babccc"), 15);
        assert_eq!(count_valid_substrings("abca"), 8);
    }
}
