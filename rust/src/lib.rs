mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// combination: choose 2k from (n+k-1)
pub fn number_of_sets(n: i32, k: i32) -> i32 {
    let [n, k] = [n, k].map(|v| v as usize);
    dfs(n, 0, 0, k, &mut vec![vec![vec![None; 1 + k]; 1 + n]; 2]).unwrap()
}

const MOD: i32 = 1_000_000_007;

fn dfs(
    n: usize,
    inline: usize,
    curr: usize,
    k: usize,
    memo: &mut [Vec<Vec<Option<i32>>>],
) -> Option<i32> {
    if k == 0 {
        return if curr <= n { Some(1) } else { None };
    }
    if curr >= n {
        return None;
    }
    if let Some(v) = memo[inline][curr][k] {
        return if v == -1 { None } else { Some(v) };
    }
    // skip this point
    let mut res = dfs(n, inline, 1 + curr, k, memo).unwrap_or(0);
    if inline == 0 {
        // start new segment
        if let Some(v) = dfs(n, 1, 1 + curr, k, memo) {
            res = (res + v) % MOD;
        }
    } else {
        // end current segment
        if let Some(v) = dfs(n, 0, curr, k - 1, memo) {
            res = (res + v) % MOD;
        }
    }
    if res == 0 {
        memo[inline][curr][k] = Some(-1);
        None
    } else {
        memo[inline][curr][k] = Some(res);
        Some(res)
    }
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
        assert_eq!(number_of_sets(4, 2), 5);
        assert_eq!(number_of_sets(3, 1), 3);
        assert_eq!(number_of_sets(30, 7), 796297179);
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
