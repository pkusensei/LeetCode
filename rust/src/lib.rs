mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_partitions_after_operations(s: &str, k: i32) -> i32 {
    dfs(s.as_bytes(), k as u32, 0, 0, false, &mut HashMap::new())
}

fn dfs(
    s: &[u8],
    k: u32,
    idx: usize,
    mask: i32,
    changed: bool,
    memo: &mut HashMap<(usize, i32, bool), i32>,
) -> i32 {
    if idx >= s.len() {
        return 1;
    }
    let key = (idx, mask, changed);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let bit = 1 << (s[idx] - b'a');
    let mut res = if (mask | bit).count_ones() <= k {
        dfs(s, k, 1 + idx, mask | bit, changed, memo)
    } else {
        1 + dfs(s, k, 1 + idx, bit, changed, memo)
    };
    if !changed {
        for i in 0..26 {
            let bit = 1 << i;
            let curr = if (mask | bit).count_ones() <= k {
                dfs(s, k, 1 + idx, mask | bit, true, memo)
            } else {
                1 + dfs(s, k, 1 + idx, bit, true, memo)
            };
            res = res.max(curr);
        }
    }
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
        assert_eq!(max_partitions_after_operations("accca", 2), 3);
        assert_eq!(max_partitions_after_operations("aabaab", 3), 1);
        assert_eq!(max_partitions_after_operations("xxyz", 1), 4);
    }

    #[test]
    fn test() {}
}
