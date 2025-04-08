mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_string(a: &str, b: &str, c: &str) -> String {
    use itertools::{Itertools, chain};
    let res = chain!(a.bytes(), b.bytes(), c.bytes()).collect_vec();
    let s = [a.as_bytes(), b.as_bytes(), c.as_bytes()]
        .iter()
        .permutations(3)
        .map(|v| {
            let [a, b, c] = v[..] else { unreachable!() };
            let v = build(a, b, c);
            dbg!(v)
        })
        .min_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)))
        .unwrap_or(res);
    String::from_utf8(s).unwrap()
}

fn build(a: &[u8], b: &[u8], c: &[u8]) -> Vec<u8> {
    let mut res = a.to_vec();
    if !is_sub(&res, b) {
        append(&mut res, b);
    }
    if !is_sub(&res, c) {
        append(&mut res, c);
    }
    res
}

fn is_sub(mut hay: &[u8], needle: &[u8]) -> bool {
    while !hay.is_empty() {
        if hay.starts_with(needle) {
            return true;
        }
        hay = &hay[1..];
    }
    false
}

fn append(res: &mut Vec<u8>, b: &[u8]) {
    let n = b.len();
    let mut flag = false;
    for i in (0..n).rev() {
        if res.ends_with(&b[..=i]) {
            res.extend_from_slice(&b[1 + i..]);
            flag = true;
            break;
        }
    }
    if !flag {
        res.extend_from_slice(b);
    }
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
        assert_eq!(minimum_string("abc", "bca", "aaa"), "aaabca");
        assert_eq!(minimum_string("ab", "ba", "aba"), "aba");
    }

    #[test]
    fn test() {
        assert_eq!(minimum_string("cac", "b", "a"), "bcac")
    }
}
