mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn frog_position(n: i32, edges: &[[i32; 2]], t: i32, target: i32) -> f64 {
    let mut adj = edges
        .iter()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, e| {
            acc.entry(e[0]).or_default().push(e[1]);
            acc.entry(e[1]).or_default().push(e[0]);
            acc
        });
    adj.entry(1).or_default().push(0); // push 0 to node 1 to compensate len-1
    let mut queue = VecDeque::from([(1, 0, 1.0)]);
    let mut seen = vec![false; 1 + n as usize];
    seen[0] = true;
    seen[1] = true;
    while let Some((node, time, prob)) = queue.pop_front() {
        if time > t {
            break;
        }
        if node == target {
            if time < t && adj.get(&node).is_some_and(|v| v.len() > 1) {
                break;
            }
            return prob; // time==t
        }
        let Some(v) = adj.get(&node) else {
            continue;
        };
        let len = v.len() as f64 - 1.0;
        for &next in v.iter() {
            if !seen[next as usize] {
                seen[next as usize] = true;
                queue.push_back((next, 1 + time, prob / len));
            }
        }
    }
    0.0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            frog_position(7, &[[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]], 2, 4),
            0.16666666666666666
        );
        assert_eq!(
            frog_position(7, &[[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]], 1, 7),
            0.3333333333333333
        );
    }

    #[test]
    fn test() {
        float_eq(
            frog_position(7, &[[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]], 20, 6),
            0.16667,
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
