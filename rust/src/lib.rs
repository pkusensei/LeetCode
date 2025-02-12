mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_flips(s: &str) -> i32 {
    let mut res = i32::MAX;
    let [mut zero_even, mut zero_odd] = [0, 0];
    let n = s.len();
    // Slide a window with length n in total 2n array
    for (idx, b) in s.bytes().cycle().enumerate().take(2 * n) {
        // Try fit all zeros onto even idx, ones onto odd idx
        zero_even += i32::from(b != (b'0' + (idx & 1) as u8));
        // Try fit all ones onto even idx, zeros onto odd idx
        zero_odd += i32::from(b != (b'0' + 1 - (idx & 1) as u8));
        if idx >= n {
            let left = idx - n; // droppped out of window
            zero_even -= i32::from(s.as_bytes()[left] != (b'0' + (left & 1) as u8));
            zero_odd -= i32::from(s.as_bytes()[left] != (b'0' + 1 - (left & 1) as u8));
        }
        if idx >= n - 1 {
            res = res.min(zero_even).min(zero_odd);
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
        assert_eq!(min_flips("111000"), 2);
        assert_eq!(min_flips("010"), 0);
        assert_eq!(min_flips("1110"), 1);
    }

    #[test]
    fn test() {
        assert_eq!(min_flips("01001001101"), 2);
    }
}
