mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn magnificent_sets(n: i32, edges: &[[i32; 2]]) -> i32 {
    let n = n as usize;
    // Thought there's some smart tricks on using DSU
    // Instead it's just here to partition the graph into sub-graphs
    let mut dsu = DSU::new(n);
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|v| e[v] as usize - 1);
        dsu.union(a, b);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut component_groups = HashMap::new();
    // Was scratching my head to find the correct start node for BFS
    // Instead every node was used to do BFS from
    for node in 0..n {
        let Some(num) = bfs(&adj, node) else {
            return -1;
        };
        let root = dsu.find(node);
        let v = component_groups.entry(root).or_insert(0);
        *v = (*v).max(num);
    }
    component_groups.into_values().sum()
}

fn bfs(adj: &[Vec<usize>], start: usize) -> Option<i32> {
    let n = adj.len();
    let mut queue = VecDeque::from([start]);
    let mut seen = vec![-1; n];
    let mut layer_count = 0;
    seen[start] = layer_count;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let curr = queue.pop_front().unwrap();
            for &next in adj[curr].iter() {
                if seen[next] == -1 {
                    seen[next] = 1 + layer_count;
                    queue.push_back(next);
                } else {
                    // curr node is of this count
                    // which cannot be shared with its neighbor
                    if seen[next] == layer_count {
                        return None;
                    }
                }
            }
        }
        layer_count += 1;
    }
    Some(layer_count)
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
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
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
            magnificent_sets(6, &[[1, 2], [1, 4], [1, 5], [2, 6], [2, 3], [4, 6]]),
            4
        );
        assert_eq!(magnificent_sets(3, &[[1, 2], [2, 3], [3, 1]]), -1);
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
