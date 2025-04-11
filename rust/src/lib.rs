mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    (low..=high).filter(|&v| check(v)).count() as i32
}

fn check(num: i32) -> bool {
    if (1 + num.ilog10()) & 1 == 1 {
        return false;
    }
    let s = num.to_string();
    let n = s.len();
    let a: u8 = s[..n / 2].bytes().map(|b| b - b'0').sum();
    let b: u8 = s[n / 2..].bytes().map(|b| b - b'0').sum();
    a == b
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
