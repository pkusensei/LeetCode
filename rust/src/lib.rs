mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_ways(words: &[&str], target: &str) -> i32 {
    let (wn, tn) = (words[0].len(), target.len());
    let mut freq: Vec<_> = std::iter::repeat([0; 26]).take(wn).collect();
    for word in words.iter() {
        for (i, b) in word.bytes().enumerate() {
            freq[i][usize::from(b - b'a')] += 1;
        }
    }
    let mut prev = vec![0; 1 + tn];
    prev[0] = 1;
    for fi in 1..=wn {
        let mut curr = prev.clone();
        for ti in 1..=tn {
            let pos = usize::from(target.as_bytes()[ti - 1] - b'a');
            curr[ti] += freq[fi - 1][pos] * prev[ti - 1];
            curr[ti] %= MOD;
        }
        prev = curr;
    }
    prev[tn] as _
    // dfs(&freq, target.as_bytes(), 0, 0, &mut vec![vec![-1; tn]; wn]) as _
}

const MOD: i64 = 1_000_000_007;

fn dfs(freq: &[[i64; 26]], target: &[u8], fi: usize, ti: usize, memo: &mut [Vec<i64>]) -> i64 {
    if ti == target.len() {
        return 1;
    }
    if freq.len() - fi < target.len() - ti {
        return 0;
    }
    if memo[fi][ti] > -1 {
        return memo[fi][ti];
    }
    let byte = usize::from(target[ti] - b'a');
    let mut res = dfs(freq, target, 1 + fi, ti, memo);
    res += freq[fi][byte] * dfs(freq, target, 1 + fi, 1 + ti, memo);
    res %= MOD;
    memo[fi][ti] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_ways(&["acca", "bbbb", "caca"], "aba"), 6);
        assert_eq!(num_ways(&["abba", "baab"], "bab"), 4);
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
