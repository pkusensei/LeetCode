mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value(n: String, x: i32) -> String {
    let x = x as u8 + b'0';
    let mut s = n.into_bytes();
    if s.first().is_some_and(|&b| b == b'-') {
        if let Some(i) = s.iter().position(|&b| b > x) {
            s.insert(i, x);
        } else {
            s.push(x);
        }
    } else if let Some(i) = s.iter().position(|&b| b < x) {
        s.insert(i, x);
    } else {
        s.push(x);
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
