mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_be_equal(s1: &str, s2: &str) -> bool {
    let [s1, s2] = [&s1, &s2].map(|s| {
        let [mut odd, mut even] = [0, 0];
        for (idx, b) in s.bytes().enumerate() {
            if idx & 1 == 0 {
                even |= 1 << (b - b'a')
            } else {
                odd |= 1 << (b - b'a');
            }
        }
        [odd, even]
    });
    s1 == s2
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
        assert!(!can_be_equal("abcd", "dacb"))
    }

    #[test]
    fn test() {}
}
