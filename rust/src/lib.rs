mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn is_bipartite(graph: &[&[i32]]) -> bool {
    let n = graph.len();
    let (mut set1, mut set2) = (HashSet::new(), HashSet::new());
    let mut seen = vec![false; n];
    for n in 0..n {
        if !seen[n] && !dfs(graph, n as i32, &mut set1, &mut set2, &mut seen) {
            return false;
        }
    }
    true
}

fn dfs(
    graph: &[&[i32]],
    curr: i32,
    set1: &mut HashSet<i32>,
    set2: &mut HashSet<i32>,
    seen: &mut [bool],
) -> bool {
    if set2.contains(&curr) {
        return false;
    }
    set1.insert(curr);
    seen[curr as usize] = true;
    for &n in graph[curr as usize].iter() {
        if seen[n as usize] && set1.contains(&n) {
            return false;
        }
        // switch affliation
        if !seen[n as usize] && !dfs(graph, n, set2, set1, seen) {
            return false;
        }
    }
    true
}

fn bfs(graph: &[&[i32]]) -> bool {
    let n = graph.len();
    let mut colors = vec![-1; n];
    for root in 0..n {
        if colors[root] > -1 {
            continue;
        }
        let mut queue = std::collections::VecDeque::from([root]);
        colors[root] = 1;
        while let Some(node) = queue.pop_front() {
            let curr = colors[node];
            for &next in graph[node].iter() {
                let v = &mut colors[next as usize];
                if *v == curr {
                    return false;
                }
                if *v == -1 {
                    *v = 1 - curr;
                    queue.push_back(next as usize);
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!bfs(&[&[1, 2, 3], &[0, 2], &[0, 1, 3], &[0, 2]]));
        debug_assert!(bfs(&[&[1, 3], &[0, 2], &[1, 3], &[0, 2]]));
    }

    #[test]
    fn test() {
        debug_assert!(bfs(&[&[1], &[0, 3], &[3], &[1, 2]]));
        debug_assert!(!bfs(&[
            &[],
            &[2, 4, 6],
            &[1, 4, 8, 9],
            &[7, 8],
            &[1, 2, 8, 9],
            &[6, 9],
            &[1, 5, 7, 8, 9],
            &[3, 6, 9],
            &[2, 3, 4, 6, 9],
            &[2, 4, 5, 6, 7, 8]
        ]))
    }

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
