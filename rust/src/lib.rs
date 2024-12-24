mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    let [n1, n2] = [&edges1, &edges2].map(|v| 1 + v.len());
    let adj1 = build(n1, &edges1);
    let adj2 = build(n2, &edges2);
    let d1 = dfs(&adj1, 0, n1)[0];
    let d2 = dfs(&adj2, 0, n2)[0];
    let combined = 1 + [d1, d2]
        .map(|v| v / 2 + i32::from(v & 1 > 0))
        .into_iter()
        .sum::<i32>();
    d1.max(d2).max(combined)
}

fn topo_sort(n: usize, adj: &[Vec<usize>]) -> i32 {
    let mut degrees: Vec<_> = adj.iter().map(|v| v.len()).collect();
    // Collect leaves to be removed
    let mut leaves: VecDeque<_> = degrees
        .iter()
        .enumerate()
        .filter_map(|(node, &deg)| if deg == 1 { Some(node) } else { None })
        .collect();
    let mut remaining_nodes = n;
    let mut removed_layers = 0;
    while remaining_nodes > 2 {
        let size = leaves.len();
        remaining_nodes -= size;
        // Remove one layer reduces diameter by 2
        removed_layers += 1;
        for _ in 0..size {
            let Some(leaf) = leaves.pop_front() else {
                continue;
            };
            for &neighbor in adj[leaf].iter() {
                degrees[neighbor] -= 1;
                if degrees[neighbor] == 1 {
                    leaves.push_back(neighbor);
                }
            }
        }
    }
    if remaining_nodes == 2 {
        1 + 2 * removed_layers
    } else {
        2 * removed_layers
    }
}

fn bfs(n: usize, adj: &[Vec<usize>]) -> i32 {
    fn farthest_node(n: usize, adj: &[Vec<usize>], node: usize) -> (usize, i32) {
        let mut queue = VecDeque::from([node]);
        let mut visited = vec![false; n];
        visited[node] = true;
        let mut farthest = 0;
        let mut max_dist = 0;
        while !queue.is_empty() {
            let size = queue.len();
            // Iterate thru all neighbors until depletion
            // The last node is "farthest" leaf
            for _ in 0..size {
                let Some(curr) = queue.pop_front() else {
                    continue;
                };
                farthest = curr;
                for &neighbor in adj[curr].iter() {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
            // diameter == from one "farthest" leaf to another
            if !queue.is_empty() {
                max_dist += 1;
            }
        }
        (farthest, max_dist)
    }

    let (farthest, _) = farthest_node(n, adj, 0);
    farthest_node(n, adj, farthest).1
}

fn dfs(adj: &[Vec<usize>], node: usize, parent: usize) -> [i32; 2] {
    let [mut depth1, mut depth2] = [0; 2]; // record 2 biggest depths
    let mut diameter = 0;
    for &neighbor in adj[node].iter() {
        if neighbor == parent {
            continue;
        }
        let [dia, mut dep] = dfs(adj, neighbor, node);
        diameter = diameter.max(dia);
        dep += 1; // include neighbor-node edge
        if dep > depth1 {
            depth2 = depth1;
            depth1 = dep;
        } else if dep > depth2 {
            depth2 = dep
        }
    }
    diameter = diameter.max(depth1 + depth2);
    [diameter, depth1]
}

fn build(n: usize, edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
    let mut res = vec![vec![]; n];
    for e in edges.iter() {
        let [e1, e2] = [e[0], e[1]].map(|v| v as usize);
        res[e1].push(e2);
        res[e2].push(e1);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            minimum_diameter_after_merge(
                vec![vec![0, 1], vec![0, 2], vec![0, 3]],
                vec![vec![0, 1]]
            ),
            3
        );
        assert_eq!(
            minimum_diameter_after_merge(
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 4],
                    vec![2, 5],
                    vec![3, 6],
                    vec![2, 7]
                ],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 4],
                    vec![2, 5],
                    vec![3, 6],
                    vec![2, 7]
                ]
            ),
            5
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

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
