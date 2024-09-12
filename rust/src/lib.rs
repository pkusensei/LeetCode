mod helper;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[allow(unused_imports)]
use helper::*;

pub fn kth_smallest(matrix: &[&[i32]], mut k: i32) -> i32 {
    let n = matrix.len();
    if n == 1 {
        return matrix[0][0];
    }
    let mut heap = BinaryHeap::from([Reverse((matrix[0][0], 0, 0))]);
    let mut seen = HashSet::new();
    let mut res = 0;
    while k > 0 && !heap.is_empty() {
        let Reverse(curr @ (num, row, col)) = heap.pop().unwrap();
        if !seen.insert(curr) {
            continue;
        }
        k -= 1;
        res = num;
        if let Some(r) = matrix.get(row + 1) {
            heap.push(Reverse((r[col], row + 1, col)));
        }
        if let Some(&n) = matrix[row].get(col + 1) {
            heap.push(Reverse((n, row, col + 1)));
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
            kth_smallest(&[&[1, 5, 9], &[10, 11, 13], &[12, 13, 15]], 8),
            13
        );
        debug_assert_eq!(kth_smallest(&[&[-5]], 1), -5);
    }

    #[test]
    fn test() {
        debug_assert_eq!(kth_smallest(&[&[1, 2], &[1, 3]], 2), 1)
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
