mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(s: &str) -> i64 {
    let n = s.len();
    let mut prefix = vec![0_i64];
    let mut prev = s.as_bytes()[0];
    for (i, b) in s.bytes().enumerate().skip(1) {
        // i is the length of previous substr
        if prev == b {
            // no op required to make substr ending in b
            prefix.push(*prefix.last().unwrap_or(&0));
        } else {
            // flip length of i to end in b
            prefix.push(i as i64 + prefix.last().unwrap_or(&0));
        }
        prev = b;
    }
    let mut suffix = vec![0_i64];
    prev = s.as_bytes()[n - 1];
    for (i, b) in s.bytes().rev().enumerate().skip(1) {
        if prev == b {
            suffix.push(*suffix.last().unwrap_or(&0));
        } else {
            suffix.push(i as i64 + suffix.last().unwrap_or(&0));
        }
        prev = b;
    }
    suffix.reverse();
    let mut res = i64::MAX;
    for (p, s) in prefix.into_iter().zip(suffix) {
        res = res.min(p + s);
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
        assert_eq!(minimum_cost("0011"), 2);
        assert_eq!(minimum_cost("010101"), 9);
    }

    #[test]
    fn test() {}
}
