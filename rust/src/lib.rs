mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn eventual_safe_nodes(graph: &[&[i32]]) -> Vec<i32> {
    let n = graph.len();
    // ins: reversed graph
    // outs: out degrees
    // queue: current "terminal"/safe nodes
    let (ins, mut outs, mut queue) = graph.iter().enumerate().fold(
        (vec![vec![]; n], vec![0; n], VecDeque::new()),
        |(mut ins, mut outs, mut queue), (i, v)| {
            if v.is_empty() {
                queue.push_back(i);
            }
            outs[i] = v.len();
            for &item in v.iter() {
                ins[item as usize].push(i);
            }
            (ins, outs, queue)
        },
    );
    let mut res = vec![];
    while let Some(node) = queue.pop_front() {
        res.push(node);
        for &v in ins[node].iter() {
            let neighbor = v as usize;
            outs[neighbor] -= 1;
            if outs[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }
    res.sort_unstable();
    res.into_iter().map(|n| n as i32).collect()
}

fn with_dfs(graph: &[&[i32]]) -> Vec<i32> {
    let n = graph.len();
    let (mut states, mut res) = (vec![None; n], vec![]);
    for i in 0..n {
        if dfs(graph, i, &mut states) {
            res.push(i as i32);
        }
    }
    res
}

fn dfs(graph: &[&[i32]], node: usize, states: &mut [Option<bool>]) -> bool {
    if let Some(v) = states[node] {
        return v;
    }
    states[node] = Some(false); // use false to detect cycles
    for &n in graph[node].iter() {
        if !dfs(graph, n as usize, states) {
            return false;
        }
    }
    states[node] = Some(true);
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            with_dfs(&[&[1, 2], &[2, 3], &[5], &[0], &[5], &[], &[]]),
            [2, 4, 5, 6]
        );
        debug_assert_eq!(
            with_dfs(&[&[1, 2, 3, 4], &[1, 2], &[3, 4], &[0, 4], &[]]),
            [4]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
