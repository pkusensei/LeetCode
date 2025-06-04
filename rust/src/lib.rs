mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_special_path(edges: &[[i32; 3]], nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, e[2]));
        acc[b].push((a, e[2]));
        acc
    });
    let mut res = [0, 1];
    dfs(
        &adj,
        &nums,
        0,
        n,
        0,
        1,
        &mut HashMap::new(),
        &mut vec![0],
        &mut res,
    );
    res.to_vec()
}

fn dfs(
    adj: &[Vec<(usize, i32)>],
    nums: &[i32],
    node: usize,
    prev: usize,
    mut left: usize,
    depth: usize,
    depths: &mut HashMap<i32, usize>,
    prefix: &mut Vec<i32>,
    res: &mut [i32; 2],
) {
    let prev_depth = depths.insert(nums[node], depth).unwrap_or(0);
    left = left.max(prev_depth);
    let val = prefix.last().unwrap_or(&0) - prefix[left];
    let count = (depth - left) as i32;
    if val > res[0] {
        *res = [val, count];
    } else if val == res[0] {
        res[1] = res[1].min(count)
    }
    for &(next, w) in &adj[node] {
        if next != prev {
            prefix.push(w + prefix.last().unwrap_or(&0));
            dfs(adj, nums, next, node, left, 1 + depth, depths, prefix, res);
            prefix.pop();
        }
    }
    depths.insert(nums[node], prev_depth);
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
        assert_eq!(longest_special_path(&[[1, 0, 8]], &[2, 2]), [0, 1]);
        assert_eq!(
            longest_special_path(
                &[[0, 1, 2], [1, 2, 3], [1, 3, 5], [1, 4, 4], [2, 5, 6]],
                &[2, 1, 2, 1, 3, 1]
            ),
            [6, 2]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            longest_special_path(&[[1, 0, 7], [1, 2, 4]], &[1, 1, 3]),
            [4, 2]
        );
        assert_eq!(
            longest_special_path(&[[1, 0, 5], [2, 1, 3]], &[3, 1, 1]),
            [5, 2]
        );
    }
}
