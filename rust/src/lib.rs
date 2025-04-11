mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations_queries(n: i32, edges: &[[i32; 3]], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = n as usize;
    let mut adj = vec![vec![0; n]; n];
    let mut max_w = 0;
    for e in edges.iter() {
        adj[e[0] as usize][e[1] as usize] = e[2];
        adj[e[1] as usize][e[0] as usize] = e[2];
        max_w = max_w.max(e[2]);
    }
    // 1) sum frequencies of weights from 0 to all nodes in freq[node][w]
    let mut freq = vec![vec![0; 1 + max_w as usize]; n];
    dfs(&adj, &mut freq, 0, n);
    // 2) Tarjan's to find LCA to queried nodes
    let mut qs = vec![vec![]; n];
    for q in queries.iter() {
        qs[q[0] as usize].push(q[1] as usize);
        qs[q[1] as usize].push(q[0] as usize);
    }
    let mut dsu = DSU::new(n);
    let mut lca = HashMap::new();
    tarjans(
        &adj,
        &qs,
        0,
        &mut dsu,
        &mut vec![false; n],
        &mut vec![0; n],
        &mut lca,
    );

    let mut res = vec![];
    for q in queries.iter() {
        let [a, b] = [0, 1].map(|i| q[i] as usize);
        let c = lca[&[a, b]];
        let mut sum = 0;
        let mut max = 0;
        for w in 1..=max_w as usize {
            let curr = freq[a][w] + freq[b][w] - 2 * freq[c][w];
            sum += curr;
            max = max.max(curr);
        }
        res.push(sum - max);
    }
    res
}

fn dfs(adj: &[Vec<i32>], freq: &mut [Vec<i32>], node: usize, prev: usize) {
    let max_w = freq[0].len();
    for (next, &w) in adj[node].iter().enumerate() {
        if w > 0 && next != prev {
            freq[next][w as usize] += 1;
            for w in 0..max_w {
                freq[next][w] += freq[node][w];
            }
            dfs(adj, freq, next, node);
        }
    }
}

fn tarjans(
    adj: &[Vec<i32>],
    queries: &[Vec<usize>],
    node: usize,
    dsu: &mut DSU,
    seen: &mut [bool],
    ancestor: &mut [usize],
    lca: &mut HashMap<[usize; 2], usize>,
) {
    seen[node] = true;
    ancestor[node] = node;
    for (next, &w) in adj[node].iter().enumerate() {
        if w > 0 && !seen[next] {
            tarjans(adj, queries, next, dsu, seen, ancestor, lca);
            dsu.union(node, next);
            ancestor[dsu.find(node)] = node;
        }
    }
    for &next in queries[node].iter() {
        if seen[next] {
            let v = ancestor[dsu.find(next)];
            lca.insert([node, next], v);
            lca.insert([next, node], v);
        }
    }
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
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
            min_operations_queries(
                7,
                &[
                    [0, 1, 1],
                    [1, 2, 1],
                    [2, 3, 1],
                    [3, 4, 2],
                    [4, 5, 2],
                    [5, 6, 2]
                ],
                &[[0, 3], [3, 6], [2, 6], [0, 6]]
            ),
            [0, 0, 1, 3]
        );
        assert_eq!(
            min_operations_queries(
                8,
                &[
                    [1, 2, 6],
                    [1, 3, 4],
                    [2, 4, 6],
                    [2, 5, 3],
                    [3, 6, 6],
                    [3, 0, 8],
                    [7, 0, 2]
                ],
                &[[4, 6], [0, 4], [6, 5], [7, 4]]
            ),
            [1, 2, 2, 3]
        );
    }

    #[test]
    fn test() {}
}
