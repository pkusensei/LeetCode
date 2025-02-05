mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub const fn check_powers_of_three(mut n: i32) -> bool {
    while n % 3 != 2 && n > 0 {
        n /= 3
    }
    n == 0
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
        assert!(check_powers_of_three(12));
        assert!(check_powers_of_three(91));
        assert!(!check_powers_of_three(21));
    }

    #[test]
    fn test() {}
}
