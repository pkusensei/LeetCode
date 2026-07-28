mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;
use rand::seq::SliceRandom;

struct MajorityChecker {
    arr: Vec<i32>,
    map: HashMap<i32, Vec<usize>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let map = arr
            .iter()
            .enumerate()
            .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &v)| {
                acc.entry(v).or_default().push(i);
                acc
            });
        let n = arr.len();
        Self { arr, map }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let [left, right, thr] = [left, right, threshold].map(|v| v as usize);
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            if let Some(num) = self.arr[left..=right].choose(&mut rng)
                && let Some(arr) = self.map.get(num)
            {
                let a = arr.partition_point(|&v| v < left);
                let b = arr.partition_point(|&v| v <= right);
                if b - a >= thr {
                    return *num;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {
        let m = MajorityChecker::new(vec![2, 2, 1, 2, 1, 2, 2, 1, 1, 2]);
        assert_eq!(-1, m.query(0, 5, 6));
    }
}
