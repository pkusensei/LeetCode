mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn special_nodes(n: i32, edges: Vec<Vec<i32>>, x: i32, y: i32, z: i32) -> i32 {
    use itertools::izip;
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let [dx, dy, dz] = [x, y, z].map(|v| bfs(&adj, v as usize));
    let mut res = 0;
    for (a, b, c) in izip!(dx, dy, dz) {
        let mut v = [a, b, c];
        v.sort_unstable();
        res += i32::from(v[0].pow(2) + v[1].pow(2) == v[2].pow(2));
    }
    res
}

fn bfs(adj: &[Vec<usize>], start: usize) -> Vec<i64> {
    use std::collections::VecDeque;
    let n = adj.len();
    let mut res = vec![-1; n];
    let mut queue = VecDeque::from([(start, 0)]);
    res[start] = 0;
    while let Some((node, step)) = queue.pop_front() {
        for &next in &adj[node] {
            if res[next] == -1 {
                res[next] = 1 + step;
                queue.push_back((next, 1 + step));
            }
        }
    }
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
