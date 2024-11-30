mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_alternating_paths(
    n: i32,
    red_edges: &[[i32; 2]],
    blue_edges: &[[i32; 2]],
) -> Vec<i32> {
    let n = n as usize;
    let (mut reds, mut blues) = (vec![vec![false; n]; n], vec![vec![false; n]; n]);
    for v in red_edges.iter() {
        reds[v[0] as usize][v[1] as usize] = true;
    }
    for v in blue_edges.iter() {
        blues[v[0] as usize][v[1] as usize] = true;
    }
    let mut res = vec![-1; n];
    // (node, dist, color) => 0:red 1:blue
    let mut queue = VecDeque::from([(0, 0, 0), (0, 0, 1)]);
    let mut seen = vec![vec![false; n]; 2];
    seen[0][0] = true;
    seen[1][0] = true;
    while let Some((node, dist, color)) = queue.pop_front() {
        if res[node] == -1 {
            res[node] = dist
        }
        if color == 0 {
            for (next, b) in blues[node].iter().enumerate() {
                if *b && !seen[1][next] {
                    seen[1][next] = true;
                    queue.push_back((next, 1 + dist, 1));
                }
            }
        } else if color == 1 {
            for (next, b) in reds[node].iter().enumerate() {
                if *b && !seen[0][next] {
                    seen[0][next] = true;
                    queue.push_back((next, 1 + dist, 0));
                }
            }
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
        debug_assert_eq!(
            shortest_alternating_paths(3, &[[0, 1], [1, 2]], &[]),
            [0, 1, -1]
        );
        debug_assert_eq!(
            shortest_alternating_paths(3, &[[0, 1]], &[[2, 1]]),
            [0, 1, -1]
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            shortest_alternating_paths(3, &[[0, 1], [0, 2]], &[[1, 0]]),
            [0, 1, 1]
        );
        debug_assert_eq!(
            shortest_alternating_paths(
                5,
                &[[0, 1], [1, 2], [2, 3], [3, 4]],
                &[[1, 2], [2, 3], [3, 1]]
            ),
            [0, 1, 2, 3, 7]
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
