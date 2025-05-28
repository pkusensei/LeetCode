mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let t1 = build(&edges1, k);
    let t2 = build(&edges2, k - 1).into_iter().max().unwrap_or(0);
    t1.into_iter().map(|v| v + t2).collect()
}

fn build(edges: &[Vec<i32>], k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut counts = Vec::with_capacity(n);
    for start in 0..n {
        let mut queue = VecDeque::from([(start, 0)]);
        let mut seen = vec![false; n];
        seen[start] = true;
        let mut count = 0;
        while let Some((node, dist)) = queue.pop_front() {
            if dist > k {
                continue;
            }
            count += 1;
            for &next in &adj[node] {
                if !seen[next] {
                    seen[next] = true;
                    queue.push_back((next, 1 + dist));
                }
            }
        }
        counts.push(count);
    }
    counts
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
    fn basics() {}

    #[test]
    fn test() {}
}
