mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_length_of_optimal_compression(s: &str, k: i32) -> i32 {
    let (n, k) = (s.len(), k as usize);
    let mut memo = vec![vec![None; 1 + k]; n];
    dfs(s, 0, Some(k), &mut memo).unwrap_or(n as _)
}

fn dfs(s: &str, idx: usize, k: Option<usize>, memo: &mut [Vec<Option<i32>>]) -> Option<i32> {
    let (n, k) = (s.len(), k?);
    if idx >= n || k >= n - idx {
        return Some(0);
    }
    if let Some(v) = memo[idx][k] {
        return if v == -1 { None } else { Some(v) };
    }
    let mut count = [0; 26];
    let mut freq = 0;
    let mut res = i32::MAX;
    for (i, b) in s.bytes().enumerate().skip(idx) {
        // For each substr [idx..=i]
        let ci = usize::from(b - b'a');
        count[ci] += 1;
        // Try find the most frequent letter
        freq = freq.max(count[ci]);
        let range = 1 + i - idx;
        // Delete all else
        let del = range - freq as usize;
        if let Some(v) = dfs(s, 1 + i, k.checked_sub(del), memo) {
            res = res.min(v + len_of(freq));
        }
    }
    memo[idx][k] = if res == i32::MAX { Some(-1) } else { Some(res) };
    memo[idx][k]
}

const fn len_of(val: i32) -> i32 {
    1 + match val {
        1 => 0,
        2..=9 => 1,
        10..=99 => 2,
        _ => 3,
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(get_length_of_optimal_compression("aaabcccd", 2), 4);
        assert_eq!(get_length_of_optimal_compression("aabbaa", 2), 2);
        assert_eq!(get_length_of_optimal_compression("aaaaaaaaaaa", 0), 3);
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
