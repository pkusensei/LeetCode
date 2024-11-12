mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
    dfs(x as _, target as _, &mut HashMap::new())
}

fn dfs(x: i64, target: i64, memo: &mut HashMap<i64, i32>) -> i32 {
    if target < x {
        return (target * 2 - 1).min(2 * (x - target)) as i32;
    }
    if let Some(&v) = memo.get(&target) {
        return v;
    }
    let n = target.ilog(x);
    let pow = x.pow(1 + n);
    if pow == target {
        memo.insert(target, n as i32);
        return n as _;
    }
    let mut res = n as i32 + dfs(x, target - pow / x, memo);
    if pow - target < target {
        let large = n as i32 + 1 + dfs(x, pow - target, memo);
        res = res.min(large);
    }
    memo.insert(target, res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(least_ops_express_target(3, 19), 5);
        debug_assert_eq!(least_ops_express_target(5, 501), 8);
        debug_assert_eq!(least_ops_express_target(100, 100000000), 3);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
