mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimize_string_value(s: String) -> String {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut freq = [0; 26];
    let mut count = 0;
    for b in s.bytes() {
        if b == b'?' {
            count += 1
        } else {
            freq[usize::from(b - b'a')] += 1;
        }
    }
    let mut heap: BinaryHeap<_> = freq
        .into_iter()
        .enumerate()
        .map(|(i, f)| Reverse((f, i as u8 + b'a')))
        .collect();
    let mut chs = Vec::with_capacity(count);
    for _ in 0..count {
        let Reverse((f, b)) = heap.pop().unwrap();
        chs.push(b);
        heap.push(Reverse((1 + f, b)));
    }
    chs.sort_unstable();
    chs.reverse();
    let mut s = s.into_bytes();
    for v in s.iter_mut() {
        if *v != b'?' {
            continue;
        }
        *v = chs.pop().unwrap();
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
    fn test() {
        assert_eq!(
            minimize_string_value("abcdefghijklmnopqrstuvwxy??".into()),
            "abcdefghijklmnopqrstuvwxyaz"
        );
    }
}
