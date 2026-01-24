mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    let n = graph.len();
    let mut seen = vec![vec![false; n]; 1 << n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        let mask = 1 << i;
        queue.push_back((mask, i, 0));
        seen[mask][i] = true;
    }
    while let Some((mask, node, step)) = queue.pop_front() {
        if mask == (1 << n) - 1 {
            return step;
        }
        for &next in &graph[node] {
            let nmask = mask | (1 << next);
            if !seen[nmask][next as usize] {
                seen[nmask][next as usize] = true;
                queue.push_back((nmask, next as usize, 1 + step));
            }
        }
    }
    -1
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
