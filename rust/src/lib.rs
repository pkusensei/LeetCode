mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn garden_no_adj(n: i32, paths: &[[i32; 2]]) -> Vec<i32> {
    let graph: HashMap<i32, Vec<i32>> = paths.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v[0]).or_default().push(v[1]);
        acc.entry(v[1]).or_default().push(v[0]);
        acc
    });
    let colors = HashSet::from([1, 2, 3, 4]);
    let mut res = Vec::with_capacity(n as usize);
    for i in 0..n {
        let used = graph
            .get(&(1 + i))
            .and_then(|v| {
                v.iter()
                    .filter_map(|&neighbor| {
                        if neighbor < 1 + i {
                            Some(res[neighbor as usize - 1])
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>()
                    .into()
            })
            .unwrap_or_default();
        res.push(*colors.difference(&used).next().unwrap_or(&1));
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(garden_no_adj(3, &[[1, 2], [2, 3], [3, 1]]), [3, 4, 2]);
        // debug_assert_eq!(garden_no_adj(4, &[[1, 2], [3, 4]]), [4, 1, 4, 1]);
        debug_assert_eq!(
            garden_no_adj(4, &[[1, 2], [2, 3], [3, 4], [4, 1], [1, 3], [2, 4]]),
            [4, 3, 2, 1]
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
