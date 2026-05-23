mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_valid_subsets(parent: &[i32], nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let adj = parent
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (node, &p)| {
            if p >= 0 {
                acc[p as usize].push(node);
            }
            acc
        });
    let [skip, take] = dfs(&adj, &nums, k, 0);
    // remove empty subset
    (skip[0] + take[0] - 1).rem_euclid(M) as i32
}

const M: i64 = 1_000_000_007;
fn dfs(adj: &[Vec<usize>], nums: &[i32], k: i32, node: usize) -> [Vec<i64>; 2] {
    let num = (nums[node] % k) as usize;
    let mut skip = vec![0; k as usize];
    let mut take = vec![0; k as usize];
    skip[0] = 1;
    take[num] = 1;
    for &next in &adj[node] {
        let mut temp_skip = vec![0; k as usize];
        let mut temp_take = vec![0; k as usize];
        let [next_skip, next_take] = dfs(adj, nums, k, next);
        let k = k as usize;
        for a in 0..k {
            for b in 0..k {
                let rem = (a + b) % k;
                let count = (next_skip[b] + next_take[b]) % M;
                temp_skip[rem] = (temp_skip[rem] + skip[a] * count % M) % M;
                temp_take[rem] = (temp_take[rem] + take[a] * next_skip[b] % M) % M
            }
        }
        skip = temp_skip;
        take = temp_take;
    }
    [skip, take]
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
        assert_eq!(count_valid_subsets(&[-1, 0, 1], &[1, 2, 3], 3), 1);
        assert_eq!(count_valid_subsets(&[-1, 0, 0, 0], &[2, 1, 2, 1], 3), 2);
    }

    #[test]
    fn test() {}
}
