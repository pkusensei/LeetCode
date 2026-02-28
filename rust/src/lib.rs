mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn solve(n: i32) -> i32 {
    (1..=n).fold(0, |acc, num| {
        ((acc << (32 - num.leading_zeros())) + i64::from(num)) % M
    }) as i32
}

pub fn concatenated_binary(n: i32) -> i32 {
    let mut res = 1;
    for num in 2..=n {
        let width = 1 + num.ilog2();
        res = res * mod_pow(2, width as _) % M;
        res = (res + f(num.into())) % M
    }
    res as i32
}

const M: i64 = 1_000_000_007;

const fn f(mut num: i64) -> i64 {
    let mut res = 0;
    let mut pow = 0;
    while num > 0 {
        let bit = num & 1;
        num >>= 1;
        res = (res + bit * mod_pow(2, pow)) % M;
        pow += 1;
    }
    res
}

const fn mod_pow(b: i64, p: i64) -> i64 {
    if p == 0 {
        return 1;
    }
    if p & 1 == 0 {
        mod_pow(b * b % M, p >> 1)
    } else {
        mod_pow(b * b % M, p >> 1) * b % M
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
