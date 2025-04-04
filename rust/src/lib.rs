mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_string(s: String) -> String {
    let n = s.len();
    let mut s = s.into_bytes();
    match s.iter().position(|&b| b == b'a') {
        None => {
            for v in s.iter_mut() {
                *v -= 1;
            }
        }
        Some(a) if a > 0 => {
            for v in s[..a].iter_mut() {
                *v -= 1;
            }
        }
        _ => {
            if let Some(b) = s.iter().position(|&b| b != b'a') {
                let mut last = b;
                while s.get(last).is_some_and(|&b| b != b'a') {
                    last += 1;
                }
                for v in s[b..last].iter_mut() {
                    *v -= 1;
                }
            } else {
                s[n - 1] = b'z';
            }
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
