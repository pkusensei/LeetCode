mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut seen = std::collections::HashMap::new();
    let mut res = 0;
    for num in nums {
        let diff = k - num;
        if let Some(v) = seen.get_mut(&diff) {
            if *v > 0 {
                res += 1;
                *v -= 1
            }
            if *v == 0 {
                seen.remove(&diff);
            }
        } else {
            *seen.entry(num).or_insert(0) += 1;
        }
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
    fn basics() {}

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
