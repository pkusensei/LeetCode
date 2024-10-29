mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_path_length(graph: &[&[i32]]) -> i32 {
    let n = graph.len();
    let mut visited = vec![vec![false; n]; 1 << n];
    for i in 0..n {
        visited[1 << i][i] = true;
    }
    // multi-source BFS
    let mut queue: std::collections::VecDeque<_> = (0..n).map(|node| (1 << node, node)).collect();
    let mut count = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let Some((mask, node)) = queue.pop_front() else {
                break;
            };
            if mask == (1 << n) - 1 {
                return count;
            }
            for &neighbor in graph[node].iter() {
                let nmask = mask | (1 << neighbor);
                if !visited[nmask][neighbor as usize] {
                    visited[nmask][neighbor as usize] = true;
                    queue.push_back((nmask, neighbor as usize));
                }
            }
        }
        count += 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(shortest_path_length(&[&[1, 2, 3], &[0], &[0], &[0]]), 4);
        debug_assert_eq!(
            shortest_path_length(&[&[1], &[0, 2, 4], &[1, 3, 4], &[2], &[1, 2]]),
            4
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
