mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_longest_balanced_substring(s: &str) -> i32 {
    let [mut zeros, mut ones] = [0, 0];
    let mut prev = b'1';
    let mut res = 0;
    for b in s.bytes() {
        if b == b'0' {
            if prev == b'1' {
                res = res.max(2 * zeros.min(ones));
                zeros = 0;
                ones = 0;
            }
            zeros += 1;
        } else {
            ones += 1;
        }
        prev = b;
    }
    res = res.max(2 * zeros.min(ones));
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
        assert_eq!(find_the_longest_balanced_substring("01000111"), 6);
        assert_eq!(find_the_longest_balanced_substring("00111"), 4);
        assert_eq!(find_the_longest_balanced_substring("111"), 0);
    }

    #[test]
    fn test() {}
}
