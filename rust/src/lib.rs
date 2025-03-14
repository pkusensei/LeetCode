mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_score(nums: &[i32], edges: &[[i32; 2]]) -> i32 {
    let n = nums.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut xors = vec![0; n];
    let mut last = vec![0; n];
    let xor = dfs(nums, &adj, 0, None, &mut 0, &mut xors, &mut last);
    let mut res = i32::MAX;
    for i1 in 1..n {
        for i2 in 1 + i1..n {
            let v1 = if i2 < last[i1] {
                xor ^ xors[i1]
            } else {
                xor ^ xors[i1] ^ xors[i2]
            };
            let v2 = if i2 < last[i1] {
                xors[i1] ^ xors[i2]
            } else {
                xors[i1]
            };
            let min = xors[i2].min(v1).min(v2);
            let max = xors[i2].max(v1).max(v2);
            res = res.min(max - min);
        }
    }
    res
}

fn dfs(
    nums: &[i32],
    adj: &[Vec<usize>],
    node: usize,
    prev: Option<usize>,
    ids: &mut usize,
    xors: &mut [i32],
    last: &mut [usize],
) -> i32 {
    let mut res = nums[node];
    let curr = *ids;
    *ids += 1;
    for &next in adj[node].iter() {
        if prev.is_none_or(|v| v != next) {
            res ^= dfs(nums, adj, next, Some(node), ids, xors, last);
        }
    }
    last[curr] = *ids; // track subtree as range in array
    xors[curr] = res;
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
            minimum_score(&[1, 5, 5, 4, 11], &[[0, 1], [1, 2], [1, 3], [3, 4]]),
            9
        );
        assert_eq!(
            minimum_score(
                &[5, 5, 2, 4, 4, 2],
                &[[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]]
            ),
            0
        );
    }

    #[test]
    fn test() {}
}
