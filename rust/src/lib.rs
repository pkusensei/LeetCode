mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let mut memo = HashMap::new();
    let mut nums: Vec<_> = (lo..=hi).map(|x| (x, dfs(x, &mut memo))).collect();
    nums.sort_by_key(|v| v.1);
    nums[k as usize - 1].0
}

fn dfs(x: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if x == 1 {
        return 0;
    }
    if let Some(&v) = memo.get(&x) {
        return v;
    }
    let res = if x & 1 == 1 {
        1 + dfs(3 * x + 1, memo)
    } else {
        1 + dfs(x / 2, memo)
    };
    memo.insert(x, res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(get_kth(12, 15, 2), 13);
        assert_eq!(get_kth(7, 11, 4), 7);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
