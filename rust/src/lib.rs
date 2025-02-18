mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_non_zero_product(p: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    if p == 1 {
        return 1;
    }
    let max: i64 = (1 << p) - 1;
    let max2 = max - 1;
    let times = (1 << (p - 1)) - 1;
    (mod_pow(max2, times, MOD) * (max % MOD) % MOD) as _
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
        assert_eq!(min_non_zero_product(1), 1);
        assert_eq!(min_non_zero_product(2), 6);
        assert_eq!(min_non_zero_product(3), 1512);
    }

    #[test]
    fn test() {}
}
