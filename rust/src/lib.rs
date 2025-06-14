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
    let mut max_dist = 0;
    let mut min_count = n as i32;
    dfs(
        &adj,
        &nums,
        0,
        n,
        0,
        None,
        &mut vec![0],
        &mut HashMap::new(),
        &mut max_dist,
        &mut min_count,
    );
    vec![max_dist, min_count]
}

fn dfs(
    adj: &[Vec<(usize, i32)>],
    nums: &[i32],
    node: usize,
    prev: usize,
    mut left: usize,
    prev_dup: Option<usize>,
    prefix: &mut Vec<i32>,
    seen: &mut HashMap<i32, usize>,
    max_dist: &mut i32,
    min_count: &mut i32,
) {
    let curr_val = nums[node];
    let depth = prefix.len();
    let prev_depth = seen.insert(curr_val, depth);
    if let Some((a, b)) = prev_depth.zip(prev_dup) {
        left = left.max(a.min(b));
    }
    let curr_dist = prefix.last().unwrap_or(&0) - prefix[left];
    let curr_count = (depth - left) as i32;
    if curr_dist > *max_dist {
        *max_dist = curr_dist;
        *min_count = curr_count;
    } else if curr_dist == *max_dist {
        *min_count = (*min_count).min(curr_count);
    }
    for &(next, w) in &adj[node] {
        if next != prev {
            prefix.push(w + prefix.last().unwrap_or(&0));
            dfs(
                adj,
                nums,
                next,
                node,
                left,
                prev_dup.max(prev_depth),
                prefix,
                seen,
                max_dist,
                min_count,
            );
            prefix.pop();
        }
    }
    if let Some(p) = prev_depth {
        seen.insert(curr_val, p);
    } else {
        seen.remove(&curr_val);
    }
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
            longest_special_path(
                &[
                    [0, 1, 1],
                    [1, 2, 3],
                    [1, 3, 1],
                    [2, 4, 6],
                    [4, 7, 2],
                    [3, 5, 2],
                    [3, 6, 5],
                    [6, 8, 3]
                ],
                &[1, 1, 0, 3, 1, 2, 1, 1, 0]
            ),
            [9, 3]
        );
        assert_eq!(
            longest_special_path(&[[1, 0, 3], [0, 2, 4], [0, 3, 5]], &[1, 1, 0, 2]),
            [5, 2]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            longest_special_path(
                &[
                    [0, 3, 2],
                    [1, 7, 10],
                    [7, 5, 10],
                    [5, 6, 9],
                    [3, 6, 2],
                    [3, 2, 5],
                    [2, 4, 10],
                    [4, 8, 5]
                ],
                &[3, 3, 1, 1, 2, 1, 4, 1, 2]
            ),
            [29, 4]
        );
        assert_eq!(
            longest_special_path(&[[1, 0, 5], [2, 1, 3]], &[3, 1, 1]),
            [8, 3]
        );
        assert_eq!(
            longest_special_path(&[[0, 2, 4], [1, 2, 10], [3, 1, 5]], &[4, 5, 4, 5]),
            [15, 3]
        );
    }
}
