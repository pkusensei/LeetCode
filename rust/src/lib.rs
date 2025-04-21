mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn maximum_length(s: &str) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut lens = [const { BinaryHeap::new() }; 26];
    for ch in s.as_bytes().chunk_by(|&a, &b| a == b) {
        let idx = usize::from(ch[0] - b'a');
        lens[idx].push(Reverse(ch.len() as i32));
        if lens[idx].len() > 3 {
            lens[idx].pop();
        }
    }
    let mut res = -1;
    for heap in lens {
        let v = heap
            .into_sorted_vec()
            .into_iter()
            .map(|v| v.0)
            .collect_vec();
        match v[..] {
            [a] => res = res.max(a - 2),
            [a, b, c] if a == b && b == c => res = res.max(a),
            [a, b, ..] if a == b => res = res.max(a - 1),
            [a, b, ..] => res = res.max(a - 2).max(b),
            _ => (),
        }
    }
    if res > 0 { res } else { -1 }
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
        assert_eq!(maximum_length("aaaa"), 2);
        assert_eq!(maximum_length("abcdef"), -1);
        assert_eq!(maximum_length("abcaba"), 1);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_length("alappaaaaapttgvvvmmc"), 3);
        assert_eq!(maximum_length("aada"), 1);
    }
}
