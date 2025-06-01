mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
    let [n, m, k] = [n, m, k].map(i64::from);
    let comb = n_choose_k(n - 1, k);
    (comb * mod_pow(m - 1, n - k - 1, MOD) % MOD * m % MOD) as i32
}

const MOD: i64 = 1_000_000_007;

fn n_choose_k(n: i64, k: i64) -> i64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let f = |acc, v| acc * v % MOD;
    let numerator = (n + 1 - k..=n).fold(1, f);
    let denominator = (1..=k).fold(1, f);
    numerator * mod_pow(denominator, MOD - 2, MOD) % MOD
}

const fn mod_pow(b: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % m, exp >> 1, m)
    } else {
        mod_pow(b * b % m, exp >> 1, m) * b % m
    }
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
        assert_eq!(count_good_arrays(3, 2, 1), 4);
        assert_eq!(count_good_arrays(4, 2, 2), 6);
        assert_eq!(count_good_arrays(5, 2, 0), 2);
    }

    #[test]
    fn test() {}
}
