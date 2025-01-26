mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_vowel_strings(n: i32) -> i32 {
    (n + 1..=n + 4).product::<i32>() / 24
    // dfs(n, 0, 0)
}

// aeiou => max is 5
fn dfs(n: i32, pos: i32, prev: i32) -> i32 {
    if pos >= n {
        return 1;
    }
    let mut res = 0;
    for i in prev..5 {
        res += dfs(n, 1 + pos, i);
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
        assert_eq!(count_vowel_strings(1), 5);
        assert_eq!(count_vowel_strings(2), 15);
        assert_eq!(count_vowel_strings(33), 66045);
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
