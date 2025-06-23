mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subtree_inversion_sum(edges: &[[i32; 2]], nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    dfs(
        &adj,
        nums,
        k,
        0,
        n,
        k,
        true,
        &mut vec![vec![[None; 2]; 1 + k as usize]; n],
    )
}

fn dfs(
    adj: &[Vec<usize>],
    nums: &[i32],
    k: i32,
    node: usize,
    prev: usize,
    dist: i32,
    sign: bool,
    memo: &mut [Vec<[Option<i64>; 2]>],
) -> i64 {
    if let Some(v) = memo[node][dist as usize][usize::from(sign)] {
        return v;
    }
    let curr_val = i64::from(nums[node]) * if sign { 1 } else { -1 };
    let mut nonflip = 0;
    let mut flip = 0;
    for &next in &adj[node] {
        if next != prev {
            nonflip += dfs(adj, nums, k, next, node, (1 + dist).min(k), sign, memo);
            if dist >= k {
                flip += dfs(adj, nums, k, next, node, 1, !sign, memo);
            }
        }
    }
    let mut res = nonflip + curr_val;
    if dist >= k {
        res = res.max(flip - curr_val);
    }
    memo[node][dist as usize][usize::from(sign)] = Some(res);
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
        assert_eq!(subtree_inversion_sum(&[[0, 1], [0, 2]], &[0, -1, -2], 3), 3);
        assert_eq!(
            subtree_inversion_sum(
                &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                &[4, -8, -6, 3, 7, -2, 5],
                2
            ),
            27
        );
        assert_eq!(
            subtree_inversion_sum(&[[0, 1], [1, 2], [2, 3], [3, 4]], &[-1, 3, -2, 4, -5], 2),
            9
        );
    }

    #[test]
    fn test() {
        assert_eq!(subtree_inversion_sum(&[[0, 1]], &[4, -2], 47), 6);
    }
}
