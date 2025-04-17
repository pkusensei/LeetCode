mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_xor_product(mut a: i64, mut b: i64, n: i32) -> i32 {
    for bit in (0..n).rev() {
        let mask: i64 = 1 << bit;
        if a & mask > 0 && b & mask > 0 {
        } else if a & mask > 0 {
            if a > b {
                a ^= mask;
                b |= mask
            }
        } else if b & mask > 0 {
            if a < b {
                a |= mask;
                b ^= mask;
            }
        } else {
            a |= mask;
            b |= mask;
        }
    }
    const MOD: i64 = 1_000_000_007;
    ((a % MOD) * (b % MOD) % MOD) as i32
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
        assert_eq!(maximum_xor_product(1, 6, 3), 12);
        assert_eq!(maximum_xor_product(6, 7, 5), 930);
    }

    #[test]
    fn test() {}
}
