mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{BTreeSet, HashMap};

pub fn maximum_weight(mut intervals: Vec<[i32; 3]>) -> Vec<i32> {
    let n = intervals.len();
    let mut ids = HashMap::new();
    for (i, &int) in intervals.iter().enumerate() {
        ids.entry(int).or_insert(i);
    }
    intervals.sort_unstable();
    let res = dfs(
        &intervals,
        &ids,
        0,
        4,
        &mut vec![[const { (-1, BTreeSet::new()) }; 5]; n],
    )
    .1;
    res.into_iter().map(|v| v as i32).collect()
}

fn dfs(
    intervals: &[[i32; 3]],
    ids: &HashMap<[i32; 3], usize>,
    idx: usize,
    k: usize,
    memo: &mut [[(i64, BTreeSet<usize>); 5]],
) -> (i64, BTreeSet<usize>) {
    if k == 0 || idx >= intervals.len() {
        return (0, BTreeSet::new());
    }
    if memo[idx][k].0 > -1 {
        return memo[idx][k].clone();
    }
    let skip = dfs(intervals, ids, idx + 1, k, memo);
    // pick
    let target = intervals[idx][1];
    let i = intervals.partition_point(|&v| v[0] <= target);
    let (mut pick_val, mut pick_indices) = dfs(intervals, ids, i, k - 1, memo);
    pick_indices.insert(ids[&intervals[idx]]);
    pick_val += i64::from(intervals[idx][2]);
    let res = match pick_val.cmp(&skip.0) {
        std::cmp::Ordering::Less => skip,
        std::cmp::Ordering::Equal => (pick_val, pick_indices.min(skip.1)),
        std::cmp::Ordering::Greater => (pick_val, pick_indices),
    };
    memo[idx][k] = res.clone();
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
        assert_eq!(
            maximum_weight(vec![
                [1, 3, 2],
                [4, 5, 2],
                [1, 5, 5],
                [6, 9, 3],
                [6, 7, 1],
                [8, 9, 1]
            ]),
            [2, 3]
        );
        assert_eq!(
            maximum_weight(vec![
                [5, 8, 1],
                [6, 7, 7],
                [4, 7, 3],
                [9, 10, 6],
                [7, 8, 2],
                [11, 14, 3],
                [3, 5, 5]
            ]),
            [1, 3, 5, 6]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            maximum_weight(vec![
                [8, 15, 32],
                [20, 21, 8],
                [8, 16, 29],
                [7, 12, 50],
                [16, 25, 27],
                [12, 17, 2],
                [8, 12, 45],
                [5, 10, 50]
            ]),
            [3, 4]
        )
    }
}
