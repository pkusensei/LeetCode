mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn path_existence_queries(
    n: i32,
    nums: &[i32],
    max_diff: i32,
    queries: &[[i32; 2]],
) -> Vec<i32> {
    use itertools::Itertools;
    let n = n as usize;
    let arr = nums
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .sorted_unstable()
        .collect_vec();
    let mut indices = vec![0; n];
    for (ai, &(_, ni)) in arr.iter().enumerate() {
        indices[ni] = ai;
    }
    let h = 2 + n.ilog2() as usize;
    let mut jumps = vec![vec![0; h]; n];
    let mut left = 0;
    for right in 0..n {
        while arr[right].0 - arr[left].0 > max_diff {
            jumps[left][0] = right - 1;
            left += 1;
        }
    }
    while left < n {
        jumps[left][0] = n - 1;
        left += 1;
    }
    for right in 1..h {
        for left in 0..n {
            jumps[left][right] = jumps[jumps[left][right - 1]][right - 1];
        }
    }
    let mut res = vec![];
    for q in queries.iter() {
        let [_a, _b] = [0, 1].map(|i| q[i] as usize);
        if _a == _b {
            res.push(0);
            continue;
        }
        let a = indices[_a].min(indices[_b]);
        let b = indices[_a].max(indices[_b]);
        let mut curr = a;
        let mut steps = 0;
        for j in (0..h).rev() {
            if jumps[curr][j] < b {
                curr = jumps[curr][j];
                steps += 1 << j;
            }
        }
        res.push(if jumps[curr][0] >= b { 1 + steps } else { -1 });
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
            path_existence_queries(5, &[1, 8, 3, 4, 2], 3, &[[0, 3], [2, 4]]),
            [1, 1]
        );
        assert_eq!(
            path_existence_queries(5, &[5, 3, 1, 9, 10], 2, &[[0, 1], [0, 2], [2, 3], [4, 3]]),
            [1, 2, -1, 1]
        );
        assert_eq!(
            path_existence_queries(3, &[3, 6, 1], 1, &[[0, 0], [0, 1], [1, 2]]),
            [0, -1, -1]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            path_existence_queries(3, &[7, 4, 7], 0, &[[2, 0], [1, 1], [2, 1], [2, 2]]),
            [1, 0, -1, 0]
        );
    }
}
