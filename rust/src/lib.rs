mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn min_reorder(n: i32, connections: &[[i32; 2]]) -> i32 {
    let adj = connections
        .iter()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, e| {
            acc.entry(e[0]).or_default().push(e[1]);
            acc.entry(e[1]).or_default().push(-e[0]);
            acc
        });
    let mut queue = VecDeque::from([0]);
    let mut seen = vec![false; n as usize];
    seen[0] = true;
    let mut res = 0;
    while let Some(curr) = queue.pop_front() {
        let Some(v) = adj.get(&curr) else {
            continue;
        };
        for &next in v.iter() {
            if !seen[next.unsigned_abs() as usize] {
                if next > 0 {
                    res += 1
                }
                seen[next.unsigned_abs() as usize] = true;
                queue.push_back(next.abs());
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
        assert_eq!(min_reorder(6, &[[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]), 3);
        assert_eq!(min_reorder(5, &[[1, 0], [1, 2], [3, 2], [3, 4]]), 2);
        assert_eq!(min_reorder(3, &[[1, 0], [2, 0]]), 0);
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
