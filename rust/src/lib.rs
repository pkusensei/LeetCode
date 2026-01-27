mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn loud_and_rich(richer: &[[i32; 2]], quiet: &[i32]) -> Vec<i32> {
    use std::collections::VecDeque;
    let n = quiet.len();
    let mut adj = vec![vec![]; n];
    let mut indegs = vec![0; n];
    for e in richer.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push(b);
        indegs[b] += 1;
    }
    // (curr_node, curr_min_quiet)
    let mut queue: VecDeque<_> = indegs
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == 0 { Some((i, i)) } else { None })
        .collect();
    let mut res: Vec<_> = (0..n).collect();
    while let Some((node, min_node)) = queue.pop_front() {
        res[node] = min_node;
        for &next in &adj[node] {
            indegs[next] -= 1;
            if quiet[min_node] <= quiet[res[next]] {
                res[next] = min_node;
            }
            if indegs[next] == 0 {
                queue.push_back((next, res[next]));
            }
        }
    }
    res.iter().map(|&v| v as i32).collect()
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
    fn basics() {
        assert_eq!(
            loud_and_rich(
                &[[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]],
                &[3, 2, 5, 4, 6, 1, 7, 0]
            ),
            [5, 5, 2, 5, 4, 5, 6, 7]
        );
    }

    #[test]
    fn test() {}
}
