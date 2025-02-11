mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn remove_occurrences(s: &str, part: &str) -> String {
        let mut res = vec![];
        for b in s.bytes() {
            res.push(b);
            if res.ends_with(part.as_bytes()) {
                res.truncate(res.len() - part.len());
            }
        }
        String::from_utf8(res).unwrap()
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
        assert_eq!(remove_occurrences("daabcbaabcbc", "abc"), "dab");
        assert_eq!(remove_occurrences("axxxxyyyyb", "xy"), "ab");
    }

    #[test]
    fn test() {}
}
