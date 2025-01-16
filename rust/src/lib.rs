mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: &[[i32; 2]],
    queries: &[[i32; 2]],
) -> Vec<bool> {
    let n = num_courses as usize;
    let mut graph = prerequisites
        .iter()
        .fold(vec![vec![false; n]; n], |mut acc, p| {
            acc[p[0] as usize][p[1] as usize] = true;
            acc
        });
    for mid in 0..n {
        for src in 0..n {
            for target in 0..n {
                graph[src][target] = graph[src][target] || (graph[src][mid] && graph[mid][target]);
            }
        }
    }
    queries
        .iter()
        .map(|q| graph[q[0] as usize][q[1] as usize])
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            check_if_prerequisite(2, &[[1, 0]], &[[0, 1], [1, 0]]),
            [false, true]
        );
        assert_eq!(
            check_if_prerequisite(2, &[], &[[1, 0], [0, 1]]),
            [false, false]
        );
        assert_eq!(
            check_if_prerequisite(3, &[[1, 2], [1, 0], [2, 0]], &[[1, 0], [1, 2]]),
            [true, true]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            check_if_prerequisite(
                5,
                &[[0, 1], [1, 2], [2, 3], [3, 4]],
                &[[0, 4], [4, 0], [1, 3], [3, 0]]
            ),
            [true, false, true, false]
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
