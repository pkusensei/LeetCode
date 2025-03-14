mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn decode_message(key: String, message: String) -> String {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut val = b'a';
    for b in key.bytes() {
        if b.is_ascii_alphabetic() && !map.contains_key(&b) {
            map.insert(b, val);
            val += 1;
            if map.len() == 26 {
                break;
            }
        }
    }
    let mut s = message.into_bytes();
    for v in s.iter_mut() {
        if v.is_ascii_alphabetic() {
            *v = map[v];
        }
    }
    String::from_utf8(s).unwrap()
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
    fn basics() {}

    #[test]
    fn test() {}
}
