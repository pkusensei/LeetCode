mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn distinct_subseq_ii(s: &str) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = s.len();
    let mut dp: Vec<i32> = vec![0; 1 + n];
    // dp[1+i] is the count of subseqs that end with [i]
    // i.e its length is 1+i
    // dp[0] => empty str has no subseq
    let mut last: HashMap<u8, usize> = HashMap::new();
    for (idx, b) in s.bytes().enumerate() {
        if let Some(&v) = last.get(&b) {
            // Similaly double the count
            // But current letter was last seen at v
            // dp[v] sebseqs are double counted
            dp[1 + idx] = (2 * dp[idx] - dp[v]).rem_euclid(MOD);
        } else {
            // Current letter is first seen
            // 1 => this letter
            // 2*dp[idx] => previous subseqs
            //              previous subseqs with current letter
            dp[1 + idx] = (1 + 2 * dp[idx]).rem_euclid(MOD);
        }
        last.insert(b, idx);
    }
    dp[n]
}

fn editorial(s: &str) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let [mut prev, mut curr] = [1_i32; 2];
    let mut last = [0; 26];
    for b in s.bytes() {
        let idx = usize::from(b - b'a');
        curr = (2 * prev - last[idx]).rem_euclid(MOD);
        last[idx] = prev;
        prev = curr;
    }
    (curr - 1).rem_euclid(MOD) // remove empty subseq
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(editorial("aba"), 6);
        debug_assert_eq!(editorial("aaa"), 3);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
