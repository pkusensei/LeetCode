mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    let [n, k] = [n, k].map(|v| v as usize);
    let mut in_degs = vec![0; n];
    let mut adj = vec![vec![]; n];
    for inv in invocations.iter() {
        let [a, b] = [0, 1].map(|i| inv[i] as usize);
        in_degs[b] += 1;
        adj[a].push(b);
    }
    let mut queue = VecDeque::from([k]);
    let mut sus = vec![false; n];
    sus[k] = true;
    while let Some(node) = queue.pop_front() {
        for &next in &adj[node] {
            in_degs[next] -= 1;
            if !sus[next] {
                sus[next] = true;
                queue.push_back(next);
            }
        }
    }
    let mut res = vec![];
    for (i, &v) in sus.iter().enumerate() {
        if v && in_degs[i] > 0 {
            return (0..n as i32).collect();
        } else if !v {
            res.push(i as i32);
        }
    }
    res
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
