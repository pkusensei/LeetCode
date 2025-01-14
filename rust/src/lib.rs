mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn number_ways(hats: &[&[i32]]) -> i32 {
    let n = hats.len();
    let hat_people =
        hats.iter()
            .enumerate()
            .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, v)| {
                for &h in v.iter() {
                    acc.entry(h as usize).or_default().push(i);
                }
                acc
            });
    // dfs(&hat_people, n, 0, 0, &mut vec![vec![-1; 1 << n]; 41])
    let done = (1 << n) - 1;
    let mut dp = vec![vec![0; 1 + done]; 42];
    for v in dp.iter_mut() {
        v[done] = 1;
    }
    for hat in (1..=40).rev() {
        for mask in (0..=done).rev() {
            let mut res = dp[1 + hat][mask];
            if let Some(v) = hat_people.get(&hat) {
                for &p in v.iter() {
                    if (mask >> p) & 1 == 0 {
                        res += dp[1 + hat][mask | (1 << p)];
                        res %= MOD;
                    }
                }
            }
            dp[hat][mask] = res;
        }
    }
    dp[1][0]
}

const MOD: i32 = 1_000_000_007;

fn dfs(
    hats: &HashMap<usize, Vec<usize>>,
    n: usize,
    hat: usize,
    mask: usize,
    memo: &mut [Vec<i32>],
) -> i32 {
    if mask.count_ones() as usize == n {
        return 1;
    }
    if hat > 40 {
        return 0;
    }
    if memo[hat][mask] > -1 {
        return memo[hat][mask];
    }
    // skip current hat
    let mut res = dfs(hats, n, 1 + hat, mask, memo);
    if let Some(v) = hats.get(&hat) {
        for &i in v.iter() {
            if (mask >> i) & 1 == 0 {
                let next = mask | (1 << i);
                res += dfs(hats, n, 1 + hat, next, memo);
                res %= MOD;
            }
        }
    }
    memo[hat][mask] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(number_ways(&[&[3, 4], &[4, 5], &[5]]), 1);
        assert_eq!(number_ways(&[&[3, 5, 1], &[3, 5]]), 4);
        assert_eq!(
            number_ways(&[&[1, 2, 3, 4], &[1, 2, 3, 4], &[1, 2, 3, 4], &[1, 2, 3, 4]]),
            24
        );
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
