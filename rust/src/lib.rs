mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_malware_spread(graph: &[&[i32]], initial: &[i32]) -> i32 {
    let n = graph.len();
    let mut dsu = dsu::DSU::new(n);
    for (x, row) in graph.iter().enumerate() {
        for (y, &v) in row.iter().enumerate() {
            if v == 1 {
                dsu.union(x, y);
            }
        }
    }

    // If 2 or more nodes are connected, removing any won't change result
    // Find single/isolated nodes first
    // i.e count[root(node)] == 1
    let mut count = vec![0; n];
    for &v in initial.iter() {
        count[dsu.find(v as usize)] += 1;
    }
    let mut res = -1;
    let mut size = -1;
    for &v in initial.iter() {
        let root = dsu.find(v as usize);
        if count[root] == 1 {
            let temp = dsu.get_size(root);
            if temp > size {
                size = temp;
                res = v;
            } else if temp == size && v < res {
                res = v;
            }
        }
    }
    // No isolcated/unique nodes
    // Find smallest in connected part(s)
    if res == -1 {
        res = *initial.iter().min().unwrap_or(&0);
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
            min_malware_spread(&[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1]], &[0, 2]),
            0
        );
        debug_assert_eq!(
            min_malware_spread(&[&[1, 1, 1], &[1, 1, 1], &[1, 1, 1]], &[1, 2]),
            1
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            min_malware_spread(&[&[1, 1, 0], &[1, 1, 0], &[0, 0, 1]], &[0, 1, 2]),
            2
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
