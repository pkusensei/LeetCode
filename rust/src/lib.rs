mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_numbers(n: i64) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let res = if n & 1 == 1 { 5 } else { 1 }; // last digit
    (res * mod_pow(20, n / 2, MOD) % MOD) as _
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
        assert_eq!(count_good_numbers(1), 5);
        assert_eq!(count_good_numbers(4), 400);
        assert_eq!(count_good_numbers(50), 564908303);
    }

    #[test]
    fn test() {}
}
