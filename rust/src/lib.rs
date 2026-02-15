mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn palindrome_path(n: i32, edges: &[[i32; 2]], s: &str, queries: &[&str]) -> Vec<bool> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    // [time to enter subtree, time to exit subtree]
    let [mut tin, mut tout] = [vec![0; n], vec![0; n]];
    // prefix xor from root to node using original `s`
    let mut pref_xor = vec![0; n];
    let mut depth = vec![0; n];
    let log = 1 + n.ilog2() as usize;
    let mut up = vec![vec![n; n]; log];
    let mut masks = s.bytes().map(|b| 1 << (b - b'a')).collect_vec();
    let mut timer = 0;
    #[rustfmt::skip]
    dfs(
        // Graph structure
        &adj, 0, n,
		// Euler tour
        &mut timer, &mut tin, &mut tout,
		// Binary lifting
        &mut depth, &mut up,
        // bitmasks
        &mut pref_xor, &masks,
    );
    // Prepare binary lifting table
    for i1 in 1..log {
        for i2 in 0..n {
            let mid = up[i1 - 1][i2];
            up[i1][i2] = if mid == n { n } else { up[i1 - 1][mid] };
        }
    }
    let mut ft = FenwickTree::new(n);
    let mut res = vec![];
    for q in queries.iter() {
        if let Some(qq) = q.strip_prefix("query ") {
            let (u, v) = qq.split_once(' ').unwrap();
            let [a, b] = [u, v].map(|v| v.parse::<usize>().unwrap());
            // pref_xor of node ^ all changes to node
            let pref_a = pref_xor[a] ^ ft.query(tin[a]);
            let pref_b = pref_xor[b] ^ ft.query(tin[b]);
            let lca = find_lca(a, b, log, &depth, &up);
            // mask(lca) gets cancelled during a^b
            let mask = pref_a ^ pref_b ^ masks[lca];
            res.push(mask.count_ones() <= 1);
        } else if let Some(qu) = q.strip_prefix("update ") {
            let (u, c) = qu.split_once(' ').unwrap();
            let u = u.parse::<usize>().unwrap();
            let nmask = 1 << (c.as_bytes()[0] - b'a');
            let delta = masks[u] ^ nmask;
            if delta != 0 {
                ft.range_xor(tin[u], tout[u], delta);
                masks[u] = nmask;
            }
        }
    }
    res
}

fn find_lca(mut a: usize, mut b: usize, log: usize, depth: &[i32], up: &[Vec<usize>]) -> usize {
    let n = depth.len();
    if depth[a] < depth[b] {
        std::mem::swap(&mut a, &mut b);
    }
    let mut diff = depth[a] - depth[b];
    for row in up {
        if diff == 0 {
            break;
        }
        if diff & 1 > 0 {
            a = row[a];
        }
        diff >>= 1;
    }
    if a == b {
        return a;
    }
    for row in (0..log).rev() {
        if up[row][a] != n && up[row][a] != up[row][b] {
            a = up[row][a];
            b = up[row][b];
        }
    }
    up[0][a]
}

fn dfs(
    // Graph structure
    adj: &[Vec<usize>],
    node: usize,
    prev: usize,
    // Euler tour
    timer: &mut usize,
    tin: &mut [usize],
    tout: &mut [usize],
    // Binary lifting
    depth: &mut [i32],
    up: &mut [Vec<usize>],
    // bitmasks
    pref_xor: &mut [i32],
    masks: &[i32],
) {
    let n = tin.len();
    *timer += 1; // so that fenwick tree always uses idx>0
    tin[node] = *timer;
    up[0][node] = prev;
    depth[node] = if prev == n { 0 } else { 1 + depth[prev] };
    pref_xor[node] = if prev == n {
        masks[node]
    } else {
        pref_xor[prev] ^ masks[node]
    };
    for &next in &adj[node] {
        if next != prev {
            dfs(
                adj, next, node, timer, tin, tout, depth, up, pref_xor, masks,
            );
        }
    }
    tout[node] = *timer;
}

struct FenwickTree {
    n: usize,
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            tree: vec![0; 1 + n],
        }
    }

    fn update(&mut self, mut idx: usize, val: i32) {
        while idx <= self.n {
            self.tree[idx] ^= val;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i32 {
        let mut res = 0;
        while idx > 0 {
            res ^= self.tree[idx];
            idx -= idx & idx.wrapping_neg();
        }
        res
    }

    fn range_xor(&mut self, left: usize, right: usize, val: i32) {
        if left <= right {
            self.update(left, val);
            if 1 + right <= self.n {
                self.update(1 + right, val);
            }
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
    fn basics() {}

    #[test]
    fn test() {}
}
