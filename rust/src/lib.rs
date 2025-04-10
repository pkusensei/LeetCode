mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let [n, k] = [n, target].map(i64::from);
    let low = n.min(k / 2);
    let mut res = (1 + low) * low / 2 % MOD;
    res += (k + k + n - low - 1) * (n - low) / 2 % MOD;
    (res % MOD) as i32
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
        assert_eq!(minimum_possible_sum(2, 3), 4);
        assert_eq!(minimum_possible_sum(3, 3), 8);
        assert_eq!(minimum_possible_sum(1, 1), 1);
    }

    #[test]
    fn test() {
        assert_eq!(minimum_possible_sum(1000000000, 1000000000), 750000042);
    }
}
