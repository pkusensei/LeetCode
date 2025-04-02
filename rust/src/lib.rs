mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_smallest_palindrome(s: String) -> String {
    let mut s = s.into_bytes();
    let mut left = 0;
    let mut right = s.len() - 1;
    while left <= right {
        while left <= right && s[left] == s[right] {
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
        if left > right {
            break;
        }
        let t = s[left].min(s[right]);
        s[left] = t;
        s[right] = t;
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
