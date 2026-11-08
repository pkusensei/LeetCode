mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(n: i32, colors: String, edges: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let curr = dfs(&adj, colors.as_bytes(), 0, n);
    curr.blue_tree.min(curr.red_root).min(curr.single_red_child)
}

fn dfs(adj: &[Vec<usize>], colors: &[u8], node: usize, prev: usize) -> State {
    let mut curr = State::default();
    for &next in &adj[node] {
        if next != prev {
            let sub = dfs(adj, colors, next, node);
            curr.blue_tree += sub.blue_tree;
            curr.red_root += sub.blue_tree.min(sub.red_root);
            curr.single_red_child = curr
                .single_red_child
                .min(sub.red_root.min(sub.single_red_child) - sub.blue_tree);
            curr.size += sub.size;
        }
    }
    if colors[node] == b'B' {
        curr.red_root += curr.size;
    } else {
        curr.blue_tree += curr.size;
    }
    curr.single_red_child += curr.blue_tree;
    curr
}

#[derive(Clone, Copy)]
struct State {
    blue_tree: i64,
    red_root: i64,
    single_red_child: i64,
    size: i64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            blue_tree: 0,
            red_root: 0,
            single_red_child: i64::MAX >> 2,
            size: 1,
        }
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
