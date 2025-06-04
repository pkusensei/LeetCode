mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distance_sum(m: i32, n: i32, k: i32) -> i32 {
    const M: i64 = 1_000_000_007;
    let [m, n, k] = [m, n, k].map(i64::from);
    let nn = n * m - 2;
    let kk = k - 2;
    let kk = kk.min(nn - kk);
    let f = |acc, v| acc * v % M;
    let nom = (nn - kk + 1..=nn).fold(1, f);
    let den = (1..=kk).fold(1, f);
    let factor = nom * mod_pow(den, M - 2, M) % M;
    let mut dist = 0;
    for d in 1..m {
        dist += d * (m - d) % M * n % M * n % M;
        dist %= M;
    }
    for d in 1..n {
        dist += d * (n - d) % M * m % M * m % M;
        dist %= M;
    }
    (dist * factor % M) as i32
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
        assert_eq!(distance_sum(2, 2, 2), 8);
        assert_eq!(distance_sum(1, 4, 3), 20);
    }

    #[test]
    fn test() {
        assert_eq!(distance_sum(50, 100, 1299), 995516149);
    }
}
