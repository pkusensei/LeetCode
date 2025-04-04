mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_semi_repetitive_substring(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut res = 1;
    let mut pair1 = -1; // left idx of [0,0]
    let mut pair2 = 0; // right idx of [0,0]
    for idx in 1..n {
        if s[idx] == s[idx - 1] {
            pair1 = pair2 - 1;
            pair2 = idx as i32;
        }
        res = res.max(idx as i32 - pair1);
    }
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
        assert_eq!(longest_semi_repetitive_substring("52233"), 4);
        assert_eq!(longest_semi_repetitive_substring("5494"), 4);
        assert_eq!(longest_semi_repetitive_substring("1111111"), 2);
    }

    #[test]
    fn test() {
        assert_eq!(longest_semi_repetitive_substring("0001"), 3);
    }
}
