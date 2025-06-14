mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn min_zero_array(nums: &[i32], queries: &[[i32; 3]]) -> i32 {
    let mut res = -1;
    for (pos, &num) in nums.iter().enumerate() {
        if let Some(v) = dfs(queries, 0, num, pos, &mut HashMap::new()) {
            res = res.max(v);
        } else {
            return -1;
        }
    }
    res
}

fn dfs(
    queries: &[[i32; 3]],
    idx: usize,
    curr: i32,
    pos: usize,
    memo: &mut HashMap<(usize, i32), Option<i32>>,
) -> Option<i32> {
    if curr == 0 {
        return Some(0);
    }
    if curr < 0 || idx >= queries.len() {
        return None;
    }
    if let Some(&v) = memo.get(&(idx, curr)) {
        return v;
    }
    let mut res = dfs(queries, 1 + idx, curr, pos, memo).map(|v| 1 + v);
    if (queries[idx][0] as usize..=queries[idx][1] as usize).contains(&pos) {
        let take = dfs(queries, 1 + idx, curr - queries[idx][2], pos, memo).map(|v| 1 + v);
        res = match (res, take) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(v), None) | (None, Some(v)) => Some(v),
            _ => None,
        };
    }
    memo.insert((idx, curr), res);
    res
}

pub fn jump(nums: &[i32], queries: &[[i32; 3]]) -> i32 {
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let mut reachable = vec![false; 1 + num as usize];
        reachable[num as usize] = true;
        let mut curr = 0;
        while !reachable[0] {
            let Some(&[left, right, val]) = queries.get(curr) else {
                return -1;
            };
            if (left..=right).contains(&(idx as i32)) {
                for x in 0..num {
                    reachable[x as usize] |= x + val <= num && reachable[(x + val) as usize]
                }
            }
            curr += 1;
        }
        res = res.max(curr as i32)
    }
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
            min_zero_array(&[2, 0, 2], &[[0, 2, 1], [0, 2, 1], [1, 1, 3]]),
            2
        );
        assert_eq!(min_zero_array(&[4, 3, 2, 1], &[[1, 3, 2], [0, 2, 1]]), -1);
        assert_eq!(
            min_zero_array(
                &[1, 2, 3, 2, 1],
                &[[0, 1, 1], [1, 2, 1], [2, 3, 2], [3, 4, 1], [4, 4, 1]]
            ),
            4
        );
        assert_eq!(
            min_zero_array(
                &[1, 2, 3, 2, 6],
                &[
                    [0, 1, 1],
                    [0, 2, 1],
                    [1, 4, 2],
                    [4, 4, 4],
                    [3, 4, 1],
                    [4, 4, 5]
                ]
            ),
            4
        );

        assert_eq!(jump(&[2, 0, 2], &[[0, 2, 1], [0, 2, 1], [1, 1, 3]]), 2);
        assert_eq!(jump(&[4, 3, 2, 1], &[[1, 3, 2], [0, 2, 1]]), -1);
        assert_eq!(
            jump(
                &[1, 2, 3, 2, 1],
                &[[0, 1, 1], [1, 2, 1], [2, 3, 2], [3, 4, 1], [4, 4, 1]]
            ),
            4
        );
        assert_eq!(
            jump(
                &[1, 2, 3, 2, 6],
                &[
                    [0, 1, 1],
                    [0, 2, 1],
                    [1, 4, 2],
                    [4, 4, 4],
                    [3, 4, 1],
                    [4, 4, 5]
                ]
            ),
            4
        );
    }

    #[test]
    fn test() {}
}
