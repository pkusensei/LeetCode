mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

// naive bfs
pub fn shortest_distance_after_queries(n: i32, queries: &[[i32; 2]]) -> Vec<i32> {
    let mut dists: Vec<_> = (0..n).collect();
    let mut graph: Vec<_> = (0..n - 1).map(|i| vec![1 + i]).collect();
    let mut res = Vec::with_capacity(queries.len());
    for q in queries {
        graph[q[0] as usize].push(q[1]);
        let mut queue = VecDeque::from([(q[0], dists[q[0] as usize])]);
        let mut seen = vec![false; n as usize];
        seen[q[0] as usize] = true;
        while let Some((node, mut dist)) = queue.pop_front() {
            match dist.cmp(&dists[node as usize]) {
                std::cmp::Ordering::Less => dists[node as usize] = dist,
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => dist = dists[node as usize],
            }
            if node == n - 1 {
                res.push(dist);
                break;
            }
            for &next in graph[node as usize].iter() {
                if !seen[next as usize] {
                    seen[next as usize] = true;
                    queue.push_back((next, 1 + dist));
                }
            }
        }
    }
    res
}

fn with_dp(n: i32, queries: &[[i32; 2]]) -> Vec<i32> {
    let n = n as usize;
    let mut dists: Vec<_> = (0..n).collect();
    let mut parents: Vec<_> = (1..=n).map(|i| vec![i - 1]).collect();
    let mut res = Vec::with_capacity(queries.len());
    for &q in queries.iter() {
        let [from, to] = q.map(|v| v as usize);
        parents[to].push(from);
        if dists[from] + 1 < dists[to] {
            dists[to] = 1 + dists[from];
            for tail in 1 + to..n {
                for &p in parents[tail].iter() {
                    dists[tail] = dists[tail].min(1 + dists[p]);
                }
            }
        }
        res.push(dists[n - 1]);
    }
    res.into_iter().map(|v| v as i32).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_dp(5, &[[2, 4], [0, 2], [0, 4]]), [3, 2, 1]);
        debug_assert_eq!(with_dp(4, &[[0, 3], [0, 2]]), [1, 1]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            shortest_distance_after_queries(6, &[[1, 4], [2, 4]]),
            [3, 3]
        );
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
