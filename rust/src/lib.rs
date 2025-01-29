mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn concatenated_binary(n: i32) -> i32 {
    (1..=i64::from(n))
        .fold((0, 0), |(res, mut digits), i| {
            if i & (i - 1) == 0 {
                digits += 1;
            }
            (((res << digits) + i) % MOD, digits)
        })
        .0 as _
    // dfs(i64::from(n), 1).0 as i32
}

// (current val, len of binary)
fn dfs(n: i64, curr: i64) -> (i64, u32) {
    if curr == n {
        return (curr, 1 + curr.ilog2());
    }
    let (prev_val, prev_len) = dfs(n, 1 + curr);
    let pow = mod_pow(2, prev_len);
    let res = prev_val + curr * pow % MOD;
    (res % MOD, prev_len + 1 + curr.ilog2())
}

const MOD: i64 = 1_000_000_007;

pub const fn mod_pow(mut base: i64, mut exp: u32) -> i64 {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * base) % MOD;
        }
        exp /= 2;
        base = base.pow(2) % MOD;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(concatenated_binary(1), 1);
        assert_eq!(concatenated_binary(3), 27);
        assert_eq!(concatenated_binary(12), 505379714);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
