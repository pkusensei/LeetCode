mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subsequence(s: &str, mut k: i32) -> i32 {
    let mut curr = 1;
    let mut res = 0;
    for b in s.bytes().rev() {
        if b == b'0' || curr <= k {
            k -= curr * i32::from(b - b'0');
            res += 1;
        }
        if curr <= k {
            curr *= 2;
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
        assert_eq!(longest_subsequence("1001010", 5), 5);
        assert_eq!(longest_subsequence("00101001", 6), 6);
    }

    #[test]
    fn test() {}
}
