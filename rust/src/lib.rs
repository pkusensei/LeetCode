mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn add_minimum(word: &str) -> i32 {
    let t = b"abc";
    let mut idx = 0;
    let mut res = 0;
    for b in word.bytes() {
        while t[idx] != b {
            idx = (1 + idx) % 3;
            res += 1;
        }
        idx = (1 + idx) % 3;
    }
    if idx == 0 { res } else { res + 3 - idx as i32 }
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
        assert_eq!(add_minimum("b"), 2);
        assert_eq!(add_minimum("aaa"), 6);
        assert_eq!(add_minimum("abc"), 0);
    }

    #[test]
    fn test() {}
}
