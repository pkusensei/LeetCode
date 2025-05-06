mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_length(nums: &[i32], k: i32) -> i32 {
    let k = k as usize;
    let mut dp = vec![HashMap::new(); 1 + k];
    let mut res = vec![0; 1 + k];
    for &num in nums.iter() {
        for kidx in (0..=k).rev() {
            let v = dp[kidx].entry(num).or_insert(0);
            *v = (1 + *v).max(kidx.checked_sub(1).map(|i| 1 + res[i]).unwrap_or(0));
            res[kidx] = res[kidx].max(*v);
        }
    }
    res[k]
    // let mut map = HashMap::new();
    // for &num in nums.iter() {
    //     let len = map.len();
    //     map.entry(num).or_insert(len);
    // }
    // let arr: Vec<usize> = nums.iter().map(|k| *map.get(k).unwrap_or(&0)).collect();
    // dfs(&arr, k, None, 0, &mut HashMap::new())
}

// tle's
fn dfs(
    arr: &[usize],
    k: i32,
    prev: Option<usize>,
    idx: usize,
    memo: &mut HashMap<(i32, Option<usize>, usize), i32>,
) -> i32 {
    if k < 0 {
        return -1; // remove last selection
    }
    if idx >= arr.len() {
        return 0;
    }
    let key = (k, prev, idx);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let res = if prev.is_some_and(|v| v == arr[idx]) {
        1 + dfs(arr, k, prev, 1 + idx, memo)
    } else {
        (1 + dfs(
            arr,
            k - i32::from(prev.is_some()),
            Some(arr[idx]),
            1 + idx,
            memo,
        ))
        .max(dfs(arr, k, prev, 1 + idx, memo))
    };
    memo.insert(key, res);
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
        assert_eq!(maximum_length(&[1, 2, 1, 1, 3], 2), 4);
        assert_eq!(maximum_length(&[1, 2, 3, 4, 5, 1], 0), 2);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_length(&[29, 30, 30], 0), 2);
    }
}
