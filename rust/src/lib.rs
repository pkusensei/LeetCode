mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn remove_almost_equal_characters(word: &str) -> i32 {
    let (s, n) = (word.as_bytes(), word.len());
    let mut res = 0;
    let mut idx = 1;
    while idx < n {
        if s[idx - 1].abs_diff(s[idx]) <= 1 {
            res += 1;
            idx += 1;
        }
        idx += 1;
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
        assert_eq!(remove_almost_equal_characters("aaaaa"), 2);
        assert_eq!(remove_almost_equal_characters("abddez"), 2);
        assert_eq!(remove_almost_equal_characters("zyxyxyz"), 3);
    }

    #[test]
    fn test() {}
}
