mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub fn construct_grid_layout(n: i32, edges: &[[i32; 2]]) -> Vec<Vec<i32>> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    if let Some(i) = adj.iter().position(|v| v.len() == 1) {
        build_1d(&adj, i)
    } else {
        let start = adj.iter().position(|v| v.len() == 2).unwrap();
        let first_row = find_1d(&adj, start);
        let cols = first_row.len();
        let rows = n / cols;
        let mut res = vec![vec![-1; cols]; rows];
        let mut seen = vec![false; n];
        let mut queue = VecDeque::new();
        for (i, &v) in first_row.iter().enumerate() {
            res[0][i] = v as i32;
            seen[v] = true;
            queue.push_back((0, i, v)); // (row, col, val)
        }
        while let Some((r, c, v)) = queue.pop_front() {
            let Ok([nr, nc]) = neighbors([r, c])
                .filter(|&[nr, nc]| {
                    res.get(nr)
                        .is_some_and(|row| row.get(nc).is_some_and(|&v| v == -1))
                })
                .exactly_one()
            else {
                break;
            };
            let Ok(&val) = adj[v].iter().filter(|&&next| !seen[next]).exactly_one() else {
                break;
            };
            res[nr][nc] = val as i32;
            seen[val] = true;
            queue.push_back((nr, nc, val));
        }
        res
    }
}

fn find_1d(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut queue = VecDeque::from([start]);
    let mut prev = HashMap::new();
    while let Some(node) = queue.pop_front() {
        if node != start && adj[node].len() == 2 {
            let mut res = vec![node];
            let mut curr = node;
            while curr != start {
                curr = prev[&curr];
                res.push(curr);
            }
            return res;
        }
        for &next in &adj[node] {
            if !prev.contains_key(&next) {
                prev.insert(next, node);
                queue.push_back(next);
            }
        }
    }
    unreachable!()
}

fn build_1d(adj: &[Vec<usize>], start: usize) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut queue = VecDeque::from([start]);
    while let Some(node) = queue.pop_front() {
        for &next in &adj[node] {
            if res.last().is_none_or(|&v| v as usize != next) {
                queue.push_back(next);
            }
        }
        res.push(node as i32);
    }
    vec![res]
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
            construct_grid_layout(4, &[[0, 1], [0, 2], [1, 3], [2, 3]]),
            [[1, 0], [3, 2]]
        );
        assert_eq!(
            construct_grid_layout(5, &[[0, 1], [1, 3], [2, 3], [2, 4]]),
            [[0, 1, 3, 2, 4]]
        );
        assert_eq!(
            construct_grid_layout(
                9,
                &[
                    [0, 1],
                    [0, 4],
                    [0, 5],
                    [1, 7],
                    [2, 3],
                    [2, 4],
                    [2, 5],
                    [3, 6],
                    [4, 6],
                    [4, 7],
                    [6, 8],
                    [7, 8]
                ]
            ),
            [[5, 0, 1], [2, 4, 7], [3, 6, 8]]
        );
    }

    #[test]
    fn test() {}
}
