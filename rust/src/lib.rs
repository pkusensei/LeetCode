mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_median(n: i32, edges: &[[i32; 3]], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, e[2]));
        acc[b].push((a, e[2]));
        acc
    });
    let jump = BinaryLifting::new(&adj);
    let mut res = vec![];
    for q in queries.iter() {
        let [a, b] = [0, 1].map(|i| q[i] as usize);
        let curr = jump.find_median(a, b);
        res.push(curr as i32);
    }
    res
}

struct BinaryLifting {
    up: Vec<Vec<usize>>,
    depth: Vec<i32>,
    weight: Vec<i64>,
    max_log: usize,
}

impl BinaryLifting {
    fn new(adj: &[Vec<(usize, i32)>]) -> Self {
        let n = adj.len();
        let max_log = 1 + n.ilog2() as usize;
        let mut s = Self {
            up: vec![vec![n; max_log]; n],
            depth: vec![0; n],
            weight: vec![0; n],
            max_log,
        };
        s.up[0][0] = 0;
        s.dfs(adj, 0, n);
        for i2 in 1..max_log {
            for i1 in 0..n {
                let p = s.up[i1][i2 - 1];
                if p < n {
                    s.up[i1][i2] = s.up[p][i2 - 1];
                }
            }
        }
        s
    }

    fn dfs(&mut self, adj: &[Vec<(usize, i32)>], node: usize, prev: usize) {
        for &(next, w) in &adj[node] {
            if next != prev {
                self.up[next][0] = node;
                self.depth[next] = 1 + self.depth[node];
                self.weight[next] = i64::from(w) + self.weight[node];
                self.dfs(adj, next, node);
            }
        }
    }

    fn lca(&self, mut n1: usize, mut n2: usize) -> usize {
        if self.depth[n1] > self.depth[n2] {
            std::mem::swap(&mut n1, &mut n2);
        }
        for i in 0..self.max_log {
            if ((self.depth[n2] - self.depth[n1]) >> i) & 1 == 1 {
                n2 = self.up[n2][i];
            }
        }
        if n1 == n2 {
            return n1;
        }
        for i in (0..self.max_log).rev() {
            if self.up[n1][i] != self.up[n2][i] {
                n1 = self.up[n1][i];
                n2 = self.up[n2][i];
            }
        }
        self.up[n1][0]
    }

    fn find_median(&self, n1: usize, n2: usize) -> usize {
        let lca = self.lca(n1, n2);
        let path1 = self.weight[n1] - self.weight[lca];
        let path2 = self.weight[n2] - self.weight[lca];
        let target = (1 + path1 + path2) / 2;
        if path1 >= target {
            self.binary_search(n1, lca, target)
        } else {
            self.binary_search_rev(n2, lca, target - path1)
        }
        // // First try the path from u to lca
        // let mut curr = 0;
        // while n1 != lca {
        //     if curr >= target {
        //         return n1;
        //     }
        //     let up = self.up[n1][0];
        //     curr += self.weight[n1] - self.weight[up];
        //     n1 = up;
        // }
        // if curr >= target {
        //     return lca; // Check lca itself
        // }
        // // If not found on n1->lca path, continue on lca->n2 path
        // // Need to traverse from lca to n2, but can't go down directly
        // // Instead,collect the path from n2 to lca and traverse it backwards
        // let mut path = Vec::new();
        // let mut temp = n2;
        // while temp != lca {
        //     path.push(temp);
        //     temp = self.up[temp][0];
        // }
        // // from lca towards n2
        // for &node in path.iter().rev() {
        //     let edge_weight = self.weight[node] - self.weight[self.up[node][0]];
        //     curr += edge_weight;
        //     if curr >= target {
        //         return node;
        //     }
        // }
        // n2 // unreachable
    }

    fn kth_ancestor(&self, mut node: usize, k: i32) -> usize {
        for i in 0..self.max_log {
            if (k >> i) & 1 == 1 {
                node = self.up[node][i];
            }
        }
        node
    }

    fn binary_search(&self, start: usize, lca: usize, target: i64) -> usize {
        let steps_to_lca = self.depth[start] - self.depth[lca];
        let mut left = 0;
        let mut right = steps_to_lca;
        while left < right {
            let mid = (left + right) / 2;
            let node = self.kth_ancestor(start, mid);
            let weight_so_far = self.weight[start] - self.weight[node];
            if weight_so_far >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        self.kth_ancestor(start, left)
    }

    fn binary_search_rev(&self, end: usize, lca: usize, target: i64) -> usize {
        let steps_from_lca = self.depth[end] - self.depth[lca];
        let mut left = 1; // Start from 1 because we already counted up to lca
        let mut right = steps_from_lca;
        while left <= right {
            let mid = (left + right) / 2;
            let node = self.kth_ancestor(end, steps_from_lca - mid);
            let weight_from_lca = self.weight[node] - self.weight[lca];
            if weight_from_lca >= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        self.kth_ancestor(end, steps_from_lca - left)
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
        assert_eq!(find_median(2, &[[0, 1, 7]], &[[1, 0], [0, 1]]), [0, 1]);
        assert_eq!(
            find_median(3, &[[0, 1, 2], [2, 0, 4]], &[[0, 1], [2, 0], [1, 2]]),
            [1, 0, 2]
        );
        assert_eq!(
            find_median(
                5,
                &[[0, 1, 2], [0, 2, 5], [1, 3, 1], [2, 4, 3]],
                &[[3, 4], [1, 2]]
            ),
            [2, 2]
        );
    }

    #[test]
    fn test() {}
}
