mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_visible_people(n: i32, _pos: i32, k: i32) -> i32 {
    (2 * n_choose_k((n - 1).into(), k.into()) % M) as i32
}

// c! // k! // (c-k)!
fn n_choose_k(n: i64, k: i64) -> i64 {
    let mut res = 1;
    let k = k.min(n - k);
    for i in (n - k + 1)..=n {
        res = res * i % M;
    }
    let den = (1..=k).fold(1, |acc, v| acc * v % M);
    res * mod_pow(den, M - 2) % M
}

const M: i64 = 1_000_000_007;

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
