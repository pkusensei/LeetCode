mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_malware_spread(graph: &[&[i32]], initial: &[i32]) -> i32 {
    let n = graph.len();
    let (mut curr, mut res) = (i32::MAX, 0);
    for &ini in initial.iter() {
        let mut dsu = dsu::DSU::new(n);
        for (x, row) in graph.iter().enumerate() {
            if x == ini as usize {
                continue;
            }
            for (y, &v) in row.iter().enumerate().skip(1 + x) {
                if y != ini as usize && v == 1 {
                    dsu.union(x, y);
                }
            }
        }
        let temp: i32 = initial
            .iter()
            .filter_map(|&v| {
                let v = v as usize;
                if v != ini as usize {
                    Some(dsu.find(v))
                } else {
                    None
                }
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .map(|v| dsu.get_size(v))
            .sum();
        match temp.cmp(&curr) {
            std::cmp::Ordering::Less => {
                curr = temp;
                res = ini;
            }
            std::cmp::Ordering::Equal => res = res.min(ini),
            std::cmp::Ordering::Greater => (),
        };
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
            min_malware_spread(&[&[1, 1, 0], &[1, 1, 0], &[0, 0, 1]], &[0, 1]),
            0
        );
        debug_assert_eq!(
            min_malware_spread(&[&[1, 1, 0], &[1, 1, 1], &[0, 1, 1]], &[0, 1]),
            1
        );
        debug_assert_eq!(
            min_malware_spread(
                &[&[1, 1, 0, 0], &[1, 1, 1, 0], &[0, 1, 1, 1], &[0, 0, 1, 1]],
                &[0, 1]
            ),
            1
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            min_malware_spread(
                &[
                    &[1, 0, 0, 0, 0, 0, 0, 0, 0],
                    &[0, 1, 0, 0, 0, 0, 0, 0, 0],
                    &[0, 0, 1, 0, 1, 0, 1, 0, 0],
                    &[0, 0, 0, 1, 0, 0, 0, 0, 0],
                    &[0, 0, 1, 0, 1, 0, 0, 0, 0],
                    &[0, 0, 0, 0, 0, 1, 0, 0, 0],
                    &[0, 0, 1, 0, 0, 0, 1, 0, 0],
                    &[0, 0, 0, 0, 0, 0, 0, 1, 0],
                    &[0, 0, 0, 0, 0, 0, 0, 0, 1]
                ],
                &[6, 0, 4]
            ),
            0
        )
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
