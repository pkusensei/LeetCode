mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_genetic_difference(parents: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = parents.len();
    let mut adj = vec![vec![]; n];
    let mut root = 0;
    for (node, &p) in parents.iter().enumerate() {
        if p == -1 {
            root = node;
        } else {
            adj[p as usize].push(node);
        }
    }
    let mut node_queries = vec![vec![]; n];
    for (idx, q) in queries.iter().enumerate() {
        node_queries[q[0] as usize].push((idx, q[1]));
    }
    let mut res = vec![0; queries.len()];
    dfs(&adj, &node_queries, &mut Trie::default(), &mut res, root);
    res
}

fn dfs(
    adj: &[Vec<usize>],
    queries: &[Vec<(usize, i32)>],
    trie: &mut Trie,
    res: &mut [i32],
    node: usize,
) {
    trie.increment(node as i32, 1);
    for q in queries[node].iter() {
        res[q.0] = trie.find(q.1);
    }
    for &next in adj[node].iter() {
        dfs(adj, queries, trie, res, next);
    }
    trie.increment(node as i32, -1);
}

#[derive(Debug, Clone, Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 2],
    count: i32,
}

impl Trie {
    fn increment(&mut self, num: i32, count: i32) {
        let mut curr = self;
        for i in (0..18).rev() {
            let bit = ((num >> i) & 1) as usize;
            curr = curr.nodes[bit].get_or_insert(Default::default());
            curr.count += count;
        }
    }

    fn find(&self, val: i32) -> i32 {
        let mut res = 0;
        let mut curr = self;
        for i in (0..18).rev() {
            let bit = ((val >> i) & 1) as usize;
            if let Some(v) = curr.nodes[1 - bit].as_ref().filter(|v| v.count > 0) {
                curr = v;
                res |= 1 << i;
            } else if let Some(ref v) = curr.nodes[bit] {
                curr = v;
            } else {
                break;
            }
        }
        res
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
            max_genetic_difference(&[-1, 0, 1, 1], &[[0, 2], [3, 2], [2, 5]]),
            [2, 3, 7]
        );
        assert_eq!(
            max_genetic_difference(&[3, 7, -1, 2, 0, 7, 0, 2], &[[4, 6], [1, 15], [0, 5]]),
            [6, 14, 7]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            max_genetic_difference(&[3, 3, 3, -1, 3], &[[2, 6], [2, 1], [1, 9], [2, 3], [3, 6]]),
            [5, 3, 10, 1, 5]
        );
    }
}
