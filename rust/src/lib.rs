mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_palindromes(s: &str, k: i32) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let k = k as usize;
    let mut res = 0;
    let mut idx = 0;
    while idx <= n - k {
        if is_palindrome(&s[idx..idx + k]) {
            res += 1;
            idx += k;
        } else if idx + k + 1 <= n && is_palindrome(&s[idx..idx + k + 1]) {
            res += 1;
            idx += k + 1;
        } else {
            idx += 1;
        }
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
        assert_eq!(max_palindromes("abaccdbbd", 3), 2);
        assert_eq!(max_palindromes("adbcda", 2), 0);
    }

    #[test]
    fn test() {}
}
