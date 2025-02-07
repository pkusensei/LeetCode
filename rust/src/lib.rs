mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_nice_divisors(prime_factors: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    if prime_factors <= 3 {
        return prime_factors;
    }
    let div = prime_factors / 3;
    let res = match prime_factors % 3 {
        0 => mod_pow(3, div as _, MOD),
        1 => mod_pow(3, div as u32 - 1, MOD) * 4,
        _ => mod_pow(3, div as _, MOD) * 2,
    };
    (res % MOD) as _
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
