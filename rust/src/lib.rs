mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_missing_value_subtree(parents: &[i32], nums: &[i32]) -> Vec<i32> {
    let n = parents.len();
    let mut res = vec![1; n];
    let Some(pos) = nums.iter().position(|&v| v == 1) else {
        return res;
    };
    let adj = parents
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (node, &pa)| {
            if pa >= 0 {
                acc[pa as usize].push(node);
            }
            acc
        });
    let max = *nums.iter().max().unwrap_or(&100_000);
    let mut seen = vec![false; 2 + max as usize];
    let mut node = pos as i32;
    let mut miss = 1;
    // Only the path from root to nums[node]==1 matters
    // Mark all values in this path with dfs
    // And walk upwards towards the root
    while node > -1 {
        dfs(&adj, nums, &mut seen, node as usize);
        while seen[miss as usize] {
            miss += 1;
        }
        res[node as usize] = miss;
        node = parents[node as usize]
    }
    res
}

fn dfs(adj: &[Vec<usize>], nums: &[i32], seen: &mut [bool], node: usize) {
    if seen[nums[node] as usize] {
        return;
    }
    for &next in adj[node].iter() {
        dfs(adj, nums, seen, next);
    }
    seen[nums[node] as usize] = true;
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
            smallest_missing_value_subtree(&[-1, 0, 0, 2], &[1, 2, 3, 4]),
            [5, 1, 1, 1]
        );
        assert_eq!(
            smallest_missing_value_subtree(&[-1, 0, 1, 0, 3, 3], &[5, 4, 6, 2, 1, 3]),
            [7, 1, 1, 4, 2, 1]
        );
        assert_eq!(
            smallest_missing_value_subtree(&[-1, 2, 3, 0, 2, 4, 1], &[2, 3, 4, 5, 6, 7, 8]),
            [1, 1, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test() {}
}
