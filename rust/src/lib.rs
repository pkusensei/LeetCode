mod helper;

use std::collections::{BinaryHeap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn k_smallest_pairs(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<[i32; 2]> {
    let k = k as usize;
    let mut res = Vec::with_capacity(k);
    let mut heap = BinaryHeap::from([Pair {
        i1: 0,
        i2: 0,
        n1: nums1[0],
        n2: nums2[0],
    }]);
    let mut seen = HashSet::new();
    while res.len() < k && !heap.is_empty() {
        let p @ Pair { i1, i2, n1, n2 } = heap.pop().unwrap();
        if !seen.insert(p) {
            continue;
        }
        res.push([n1, n2]);
        if i1 + 1 < nums1.len() {
            heap.push(Pair {
                i1: i1 + 1,
                i2,
                n1: nums1[i1 + 1],
                n2,
            });
        }
        if i2 + 1 < nums2.len() {
            heap.push(Pair {
                i1,
                i2: i2 + 1,
                n1,
                n2: nums2[i2 + 1],
            });
        }
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pair {
    i1: usize,
    i2: usize,
    n1: i32,
    n2: i32,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.n1 + other.n2)
            .cmp(&(self.n1 + self.n2))
            .then(other.n1.cmp(&self.n1))
            .then(other.n2.cmp(&self.n2))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            k_smallest_pairs(&[1, 7, 11], &[2, 4, 6], 3),
            [[1, 2], [1, 4], [1, 6]]
        );
        debug_assert_eq!(
            k_smallest_pairs(&[1, 1, 2], &[1, 2, 3], 2),
            [[1, 1], [1, 1]]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test() {
        debug_assert_eq!(
        k_smallest_pairs(&[1, 2, 4, 5, 6], &[3, 5, 7, 9], 20),
        [[1,3],[2,3],[1,5],[2,5],[4,3],[1,7],[5,3],[2,7],[4,5],[6,3],[1,9],[5,5],[2,9],[4,7],[6,5],[5,7],[4,9],[6,7],[5,9],[6,9]]
        )
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
