mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_substrings(s: &str, k: i32) -> i32 {
    use std::collections::HashMap;
    const V: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
    let root = (1..=k).find(|&v| v * v % k == 0).unwrap_or(k);
    let mut res = 0;
    let mut v = 0;
    let mut v_minus_c = 0;
    let mut map = HashMap::from([([0, 0], 1)]);
    for b in s.bytes() {
        if V.contains(&b) {
            v = (1 + v) % root; // count**2%k=0
            v_minus_c += 1; // v_count==c_count
        } else {
            v_minus_c -= 1;
        }
        let count = map.entry([v, v_minus_c]).or_insert(0);
        res += *count;
        *count += 1;
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
        assert_eq!(beautiful_substrings("baeyh", 2), 2);
        assert_eq!(beautiful_substrings("abba", 1), 3);
        assert_eq!(beautiful_substrings("bcdf", 1), 0);
    }

    #[test]
    fn test() {}
}
