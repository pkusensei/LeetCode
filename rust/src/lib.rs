mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn common_factors(a: i32, b: i32) -> i32 {
    const fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 { b } else { gcd(b % a, a) }
    }
    let v = gcd(a, b);
    (1..=v).filter(|&div| v % div == 0).count() as i32
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
