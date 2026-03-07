mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_flips(s: &str) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let [mut zero_even, mut zero_odd] = [0, 0];
    let mut res = n as i32;
    let put_zero_on_even = |idx, b| (idx & 1) == 0 && b == b'1' || (idx & 1) == 1 && b == b'0';
    let put_zero_on_odd = |idx, b| (idx & 1) == 0 && b == b'0' || (idx & 1) == 1 && b == b'1';
    for (idx, &b) in s.iter().cycle().enumerate().take(2 * n) {
        zero_even += i32::from(put_zero_on_even(idx, b));
        zero_odd += i32::from(put_zero_on_odd(idx, b));
        if idx >= n {
            let i = idx - n;
            zero_even -= i32::from(put_zero_on_even(i, s[i]));
            zero_odd -= i32::from(put_zero_on_odd(i, s[i]));
        }
        if idx >= n - 1 {
            res = res.min(zero_even).min(zero_odd)
        }
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(min_flips("010"), 0);
    }

    #[test]
    fn test() {
        assert_eq!(min_flips("01001001101"), 2);
    }
}
