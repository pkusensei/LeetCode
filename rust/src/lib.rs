mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_good_strings(n: i32, s1: &str, s2: &str, evil: &str) -> i32 {
    let lps = compute_lps(evil);
    dfs(
        s1.as_bytes(),
        s2.as_bytes(),
        evil.as_bytes(),
        &lps,
        &mut vec![vec![vec![vec![-1; 2]; 2]; 1 + evil.len()]; 1 + n as usize],
        0,
        0,
        true,
        true,
    )
}

fn dfs(
    s1: &[u8],
    s2: &[u8],
    evil: &[u8],
    lps: &[usize],
    memo: &mut [Vec<Vec<Vec<i32>>>],
    idx: usize,
    matched: usize,
    left_bound: bool,
    right_bound: bool,
) -> i32 {
    if matched == evil.len() {
        return 0;
    }
    if idx >= s1.len() {
        return 1;
    }
    if memo[idx][matched][usize::from(left_bound)][usize::from(right_bound)] > 0 {
        return memo[idx][matched][usize::from(left_bound)][usize::from(right_bound)];
    }
    let left = if left_bound { s1[idx] } else { b'a' };
    let right = if right_bound { s2[idx] } else { b'z' };
    let mut res = 0;
    for byte in left..=right {
        let mut next_matched = matched;
        while evil[next_matched] != byte && next_matched > 0 {
            next_matched = lps[next_matched - 1];
        }
        next_matched += usize::from(byte == evil[next_matched]);
        res += dfs(
            s1,
            s2,
            evil,
            lps,
            memo,
            1 + idx,
            next_matched,
            left_bound && byte == left,
            right_bound && byte == right,
        );
        res %= 1_000_000_007;
    }
    memo[idx][matched][usize::from(left_bound)][usize::from(right_bound)] = res;
    res
}

fn compute_lps(s: &str) -> Vec<usize> {
    let (t, n) = (s.as_bytes(), s.len());
    let mut lps = vec![0; n];
    let mut len = 0;
    let mut idx = 1;
    while idx < n {
        if t[idx] == t[len] {
            len += 1;
            lps[idx] = len;
            idx += 1
        } else if len > 0 {
            len = lps[len - 1];
        } else {
            idx += 1
        }
    }
    lps
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(find_good_strings(2, "aa", "da", "b"), 51);
        assert_eq!(find_good_strings(8, "leetcode", "leetgoes", "leet"), 0);
        assert_eq!(find_good_strings(2, "gx", "gz", "x"), 2);
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
