mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_one_bit_operations(n: i32) -> i32 {
    let mut map: HashMap<_, _> = (0..30)
        .map(|p| (2i32.pow(p), 2i32.pow(1 + p) - 1))
        .collect();
    dfs(n, &mut map)
}

fn dfs(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if n <= 1 {
        return n;
    }
    if let Some(&v) = memo.get(&n) {
        return v;
    }
    let p = n.ilog2();
    let res = dfs(2i32.pow(p), memo) - dfs(n - 2i32.pow(p), memo);
    memo.insert(n, res);
    res
}

pub fn iteration(n: i32) -> i32 {
    let mut res = 0;
    let mut k = 0;
    let mut mask = 1;
    while mask <= n {
        if (n & mask) > 0 {
            res = (1 << (1 + k)) - 1 - res;
        }
        mask <<= 1;
        k += 1;
    }
    res
}

pub fn gray_code(n: i32) -> i32 {
    let mut res = n;
    res ^= res >> 16;
    res ^= res >> 8;
    res ^= res >> 4;
    res ^= res >> 2;
    res ^= res >> 1;
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
        assert_eq!(minimum_one_bit_operations(3), 2);
        assert_eq!(minimum_one_bit_operations(6), 4);

        assert_eq!(iteration(3), 2);
        assert_eq!(iteration(6), 4);

        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(6), 4);
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
