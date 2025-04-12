mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_odd_binary_number(s: String) -> String {
    let n = s.len();
    let mut s = s.into_bytes();
    let ones = s.iter().filter(|&&v| v == b'1').count();
    s.fill(b'0');
    s[n - 1] = b'1';
    s[..ones - 1].fill(b'1');
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
