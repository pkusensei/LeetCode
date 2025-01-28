mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_effort(tasks: &mut [[i32; 2]]) -> i32 {
        tasks.sort_unstable_by_key(|t| t[1] - t[0]);
        let mut res = 0;
        for t in tasks {
            res += t[0];
            res = res.max(t[1]);
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
        assert_eq!(minimum_effort(&mut [[1, 2], [2, 4], [4, 8]]), 8);
        assert_eq!(
            minimum_effort(&mut [[1, 3], [2, 4], [10, 11], [10, 12], [8, 9]]),
            32
        );
        assert_eq!(
            minimum_effort(&mut [[1, 7], [2, 8], [3, 9], [4, 10], [5, 11], [6, 12]]),
            27
        )
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
