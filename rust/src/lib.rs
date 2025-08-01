mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn super_pow(a: i32, b: &[i32]) -> i32 {
    const M: i32 = 1337;
    // Effectively insert 0 in front of b
    let mut res = 1;
    for &d in b.iter() {
        // Suppose b=[d1, d2, ..]
        // res = pow(a, b) = pow(a, 10*d1 + d2)
        // pow(a, 10*d1) * pow(a, d2)
        // pow(pow(a, d1), 10) * pow(a, d2)
        // res *= pow(a, d1), then res = pow(res, 10);
        // res *= pow(a, d2)
        res = mod_pow(res, 10, M);
        res = res * mod_pow(a % M, d, M);
        res %= M;
    }
    res
}

const fn mod_pow(b: i32, exp: i32, m: i32) -> i32 {
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
    fn test() {
        assert_eq!(super_pow(2147483647, &[2, 0, 0]), 1198);
    }
}
