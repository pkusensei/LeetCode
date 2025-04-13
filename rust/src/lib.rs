mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_visited_nodes(edges: &[i32]) -> Vec<i32> {
    use std::collections::VecDeque;
    let n = edges.len();
    let mut indegs = edges.iter().fold(vec![0; n], |mut acc, &v| {
        acc[v as usize] += 1;
        acc
    });
    let mut queue: VecDeque<_> = indegs
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == 0 { Some(i) } else { None })
        .collect();
    let mut out_of_cycle = vec![];
    while let Some(node) = queue.pop_front() {
        out_of_cycle.push(node);
        let next = edges[node] as usize;
        indegs[next] -= 1;
        if indegs[next] == 0 {
            queue.push_back(next);
        }
    }
    let mut res = vec![0; n];
    for node in 0..n {
        if indegs[node] > 0 {
            cycle_dfs(edges, &mut indegs, node, 0, &mut res);
        }
    }
    for node in out_of_cycle {
        out_dfs(edges, node, &mut res);
    }
    res
}

fn cycle_dfs(edges: &[i32], indegs: &mut [i32], node: usize, depth: i32, res: &mut [i32]) -> i32 {
    if indegs[node] == 0 {
        res[node] = depth;
        return depth;
    }
    indegs[node] = 0; // mark visited
    let val = cycle_dfs(edges, indegs, edges[node] as usize, 1 + depth, res);
    res[node] = val;
    val
}

fn out_dfs(edges: &[i32], node: usize, res: &mut [i32]) -> i32 {
    if res[node] > 0 {
        return res[node];
    }
    let val = 1 + out_dfs(edges, edges[node] as usize, res);
    res[node] = val;
    val
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
        assert_eq!(count_visited_nodes(&[1, 2, 0, 0]), [3, 3, 3, 4]);
        assert_eq!(count_visited_nodes(&[1, 2, 3, 4, 0]), [5, 5, 5, 5, 5]);
    }

    #[test]
    fn test() {}
}
