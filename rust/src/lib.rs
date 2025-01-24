mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_subgraphs_for_each_diameter(n: i32, edges: &[[i32; 2]]) -> Vec<i32> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![false; n]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a][b] = true;
        acc[b][a] = true;
        acc
    });
    let mut res = vec![0; n - 1];
    dfs(&adj, (1 << n) - 1, &mut res, &mut vec![None; 1 << n]);
    res
}

fn dfs(adj: &[Vec<bool>], mask: u16, res: &mut [i32], memo: &mut [Option<usize>]) -> Option<usize> {
    let n = adj.len();
    // match memo[mask]
    // None => this state/mask has not been visited/processed
    // Some(v) => this mask is seen before, but the result is not necessary valid
    // i.e the mask might be representing a disconnected graph
    // Some(n) => this is processed but invalid state
    // Some(_) => this is a valid result
    if let Some(v) = memo[usize::from(mask)] {
        return if v >= n { None } else { Some(v) };
    }
    if mask.count_ones() == 2 {
        let temp: Vec<usize> = (0..16).filter(|i| mask & (1 << i) > 0).collect();
        if adj
            .get(temp[0])
            .is_some_and(|row| row.get(temp[1]).is_some_and(|&v| v))
        {
            res[0] += 1; // base case: 2 nodes are connected
            memo[usize::from(mask)] = Some(0);
            return Some(0); // dist is 1, but index from 0
        } else {
            memo[usize::from(mask)] = Some(n);
            return None;
        }
    }
    let mut max_dist = 0;
    for i in 0..n {
        if mask & (1 << i) == 0 {
            continue;
        }
        let next = mask ^ (1 << i); // remove i and try find smaller tree
        if let Some(smaller) = dfs(adj, next, res, memo) {
            if let Some(d) = bfs_dist(adj, mask, i) {
                // smaller tree is valid and i is connected to it
                let dist = (1 + smaller).min(d - 1);
                max_dist = max_dist.max(dist);
            }
        }
    }
    if max_dist == 0 {
        memo[usize::from(mask)] = Some(n);
        None
    } else {
        memo[usize::from(mask)] = Some(max_dist);
        res[max_dist] += 1;
        Some(max_dist)
    }
}

fn bfs_dist(adj: &[Vec<bool>], mask: u16, start: usize) -> Option<usize> {
    let mut queue = std::collections::VecDeque::from([(start, 1 << start, 0)]);
    let mut seen = 1 << start;
    let mut res = 0;
    while let Some((node, path, dist)) = queue.pop_front() {
        res = res.max(dist);
        for (next, &v) in adj[node].iter().enumerate() {
            // next node is in subtree/mask but not seen on this path
            if v && (mask & (1 << next)) > 0 && (path & (1 << next) == 0) {
                seen |= 1 << next;
                queue.push_back((next, path | (1 << next), 1 + dist));
            }
        }
    }
    if seen == mask {
        // all nodes can be reached => diameter
        Some(res)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            count_subgraphs_for_each_diameter(4, &[[1, 2], [2, 3], [2, 4]]),
            [3, 4, 0]
        );
        assert_eq!(count_subgraphs_for_each_diameter(2, &[[1, 2]]), [1]);
        assert_eq!(
            count_subgraphs_for_each_diameter(3, &[[1, 2], [2, 3]]),
            [2, 1]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
