mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_substrings(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut res = 0;
    for zeros in 0..=n.isqrt() {
        let mut curr = [0, 0];
        let mut left = 0;
        let mut prev = 0; // first zero in window
        for (right, &b) in s.iter().enumerate() {
            curr[usize::from(b - b'0')] += 1;
            while curr[0] > zeros {
                curr[usize::from(s[left] - b'0')] -= 1;
                left += 1;
            }
            if curr[0] == zeros && curr[1] >= zeros.pow(2) && curr[1] > 0 {
                prev = prev.max(left);
                while prev < right && s[prev] == b'1' {
                    prev += 1;
                }
                res += 1 + (prev - left).min(curr[1] - curr[0].pow(2));
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(number_of_substrings("00011"), 5);
        assert_eq!(number_of_substrings("101101"), 16);
    }

    #[test]
    fn test() {}
}
