mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn min_time(n: i32, edges: &[[i32; 2]], has_apple: &[bool]) -> i32 {
    let adj = edges
        .iter()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, e| {
            acc.entry(e[0]).or_default().push(e[1]);
            acc.entry(e[1]).or_default().push(e[0]);
            acc
        });
    dfs(&adj, has_apple, 0, -1)
    // let mut must_visit = has_apple.to_vec();
    // let mut queue = VecDeque::from([0]);
    // let mut seen = vec![false; n as usize];
    // seen[0] = true;
    // let mut prevs = HashMap::new();
    // let mut targets = vec![];
    // while let Some(node) = queue.pop_front() {
    //     if must_visit[node as usize] {
    //         must_visit[node as usize] = false;
    //         targets.push(node);
    //     }
    //     if must_visit.iter().all(|&b| !b) {
    //         break;
    //     }
    //     for &next in adj[&node].iter() {
    //         if !seen[next as usize] {
    //             seen[next as usize] = true;
    //             queue.push_back(next);
    //             prevs.insert(next, node);
    //         }
    //     }
    // }
    // seen = vec![false; n as usize];
    // seen[0] = true;
    // let mut res = 0;
    // while let Some(mut node) = targets.pop() {
    //     while !seen[node as usize] {
    //         res += 1;
    //         seen[node as usize] = true;
    //         node = prevs[&node];
    //     }
    // }
    // 2 * res
}

fn dfs(adj: &HashMap<i32, Vec<i32>>, has_apple: &[bool], node: i32, prev: i32) -> i32 {
    let mut res = 0;
    let Some(v) = adj.get(&node) else { return 0 };
    for &next in v.iter() {
        if next == prev {
            continue;
        }
        let time = dfs(adj, has_apple, next, node);
        if time > 0 || has_apple[next as usize] {
            res += time + 2;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            min_time(
                7,
                &[[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                &[false, false, true, false, true, true, false]
            ),
            8
        );
        assert_eq!(
            min_time(
                7,
                &[[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                &[false, false, true, false, false, true, false]
            ),
            6
        );
        assert_eq!(
            min_time(
                7,
                &[[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                &[false, false, false, false, false, false, false]
            ),
            0
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
