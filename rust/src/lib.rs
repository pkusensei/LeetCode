mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn build_matrix(
    k: i32,
    row_conditions: &[[i32; 2]],
    col_conditions: &[[i32; 2]],
) -> Vec<Vec<i32>> {
    let k = k as usize;
    let rseq = topo_sort(k, row_conditions);
    let cseq = topo_sort(k, col_conditions);
    if rseq.is_empty() || cseq.is_empty() {
        return vec![];
    }
    let mut res = vec![vec![0; k]; k];
    for v in 1..=k as i32 {
        let r = rseq[&v];
        let c = cseq[&v];
        res[r][c] = v;
    }
    res
}

fn topo_sort(k: usize, conds: &[[i32; 2]]) -> HashMap<i32, usize> {
    let mut indegs = vec![0; 1 + k];
    let mut adj = vec![vec![]; 1 + k];
    for c in conds {
        let [a, b] = [0, 1].map(|i| c[i] as usize);
        indegs[b] += 1;
        adj[a].push(b);
    }
    let mut queue: VecDeque<_> = indegs
        .iter()
        .enumerate()
        .filter_map(|(node, &deg)| {
            if deg == 0 && node > 0 {
                Some(node)
            } else {
                None
            }
        })
        .collect();
    let mut res = HashMap::with_capacity(k);
    while let Some(node) = queue.pop_front() {
        res.insert(node as i32, res.len());
        for &next in adj[node].iter() {
            indegs[next] -= 1;
            if indegs[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    if res.len() < k { HashMap::new() } else { res }
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
            build_matrix(3, &[[1, 2], [3, 2]], &[[2, 1], [3, 2]]),
            [[0, 0, 1], [3, 0, 0], [0, 2, 0]]
        );
        assert!(build_matrix(3, &[[1, 2], [2, 3], [3, 1], [2, 3]], &[[2, 1]]).is_empty());
    }

    #[test]
    fn test() {}
}
