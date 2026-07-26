mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_valid_sequences(n: i32, k: i32) -> i32 {
    let [n, k] = [n, k].map(i64::from);
    let a = n_choose_k(n - 1, k - 1);
    if (n - k) & 1 == 1 {
        a as i32
    } else {
        let b = n_choose_k((n + k) / 2 - 1, k - 1);
        (a - b).rem_euclid(M) as i32
    }
}

const M: i64 = 1_000_000_007;
fn n_choose_k(n: i64, k: i64) -> i64 {
    let k = k.min(n - k);
    let nom = (n - k + 1..=n).fold(1, |acc, v| acc * v % M);
    let den = (1..=k).fold(1, |acc, v| acc * v % M);
    nom * mod_pow(den, M - 2) % M
}

const fn mod_pow(b: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % M, exp >> 1)
    } else {
        mod_pow(b * b % M, exp >> 1) * b % M
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
