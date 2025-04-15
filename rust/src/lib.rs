mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_changes(s: &str, k: i32) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let k = k as usize;
    let mut divs = HashMap::new();
    for d in 2..n {
        for v in (d + d..=n).step_by(d) {
            divs.entry(v).or_insert(vec![1]).push(d);
        }
    }
    let mut memo1 = vec![vec![-1; 1 + k]; 1 + n];
    let mut memo2 = vec![vec![vec![-1; n]; 1 + n]; 1 + n];
    dfs(s, &divs, n, k, &mut memo1, &mut memo2)
}

// dp1: Break into chunks and try all substrs
fn dfs(
    s: &[u8],
    divs: &HashMap<usize, Vec<usize>>,
    idx: usize,
    k: usize,
    memo1: &mut [Vec<i32>],
    memo2: &mut [Vec<Vec<i32>>],
) -> i32 {
    if k == 1 {
        return make_semi(s, divs, 0, idx, memo2);
    }
    if memo1[idx][k] > -1 {
        return memo1[idx][k];
    }
    let mut res = i32::MAX;
    for left in 2 * (k - 1)..idx - 1 {
        res =
            res.min(dfs(s, divs, left, k - 1, memo1, memo2) + make_semi(s, divs, left, idx, memo2));
    }
    memo1[idx][k] = res;
    res
}

// dp2: For each substr [left..right), find d that minimizes changes
fn make_semi(
    s: &[u8],
    divs: &HashMap<usize, Vec<usize>>,
    left: usize,
    right: usize,
    memo2: &mut [Vec<Vec<i32>>],
) -> i32 {
    let mut res = i32::MAX;
    for &d in divs.get(&(right - left)).unwrap_or(&vec![1]) {
        res = res.min(count_diff(s, left, right, d, memo2));
    }
    res
}

fn count_diff(s: &[u8], left: usize, right: usize, d: usize, memo2: &mut [Vec<Vec<i32>>]) -> i32 {
    if left >= right {
        return 0;
    }
    if memo2[left][right][d] > -1 {
        return memo2[left][right][d];
    }
    let mut res = count_diff(s, left + d, right - d, d, memo2);
    for delta in 0..d {
        res += i32::from(s[left + delta] != s[right + delta - d]);
    }
    memo2[left][right][d] = res;
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(minimum_changes("abcac", 2), 1);
        assert_eq!(minimum_changes("abcdef", 2), 2);
        assert_eq!(minimum_changes("aabbaa", 3), 0);
    }

    #[test]
    fn test() {}
}
