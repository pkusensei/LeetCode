mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::{BTreeSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = c as usize;
    let adj = connections.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut ids = vec![n; n];
    let mut curr_id = 0;
    for start in 0..n {
        if ids[start] == n {
            let mut queue = VecDeque::from([start]);
            ids[start] = curr_id;
            while let Some(node) = queue.pop_front() {
                for &next in &adj[node] {
                    if ids[next] == n {
                        ids[next] = curr_id;
                        queue.push_back(next);
                    }
                }
            }
            curr_id += 1;
        }
    }
    let mut sets = vec![BTreeSet::new(); curr_id];
    for (node, &id) in ids.iter().enumerate() {
        sets[id].insert(node);
    }
    let mut res = vec![];
    for q in queries {
        let node = q[1] as usize - 1;
        let id = ids[node];
        if q[0] == 1 {
            if sets[id].contains(&node) {
                res.push(q[1]);
            } else {
                res.push(sets[id].first().map(|v| 1 + *v as i32).unwrap_or(-1));
            }
        } else {
            sets[id].remove(&(q[1] as usize - 1));
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
