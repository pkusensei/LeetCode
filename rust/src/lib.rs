mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_shortest_cycle(n: i32, edges: &[[i32; 2]]) -> i32 {
    use std::collections::VecDeque;
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    let mut degs = vec![0; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push(b);
        adj[b].push(a);
        degs[a] += 1;
        degs[b] += 1;
    }
    let mut nodes = vec![];
    for (node, &deg) in degs.iter().enumerate() {
        if deg == 1 {
            adj[node].clear();
        } else {
            nodes.push((node, deg));
        }
    }
    nodes.sort_unstable_by_key(|&(_, v)| v);
    let mut res = None;
    'outer: while let Some((start, _)) = nodes.pop() {
        let mut queue = VecDeque::from([(start, n, 0)]);
        while let Some((node, prev, dist)) = queue.pop_front() {
            if node == start && dist > 0 {
                let min = res.get_or_insert(dist);
                *min = (*min).min(dist);
                adj[start].clear();
                continue 'outer;
            }
            for &next in adj[node].iter() {
                if next != prev {
                    queue.push_back((next, node, 1 + dist));
                }
            }
        }
    }
    res.unwrap_or(-1)
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
            find_shortest_cycle(7, &[[0, 1], [1, 2], [2, 0], [3, 4], [4, 5], [5, 6], [6, 3]]),
            3
        );
        assert_eq!(find_shortest_cycle(4, &[[0, 1], [0, 2]]), -1);
    }

    #[test]
    fn test() {}
}
