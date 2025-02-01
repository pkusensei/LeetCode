mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(target: &[i32], arr: &[i32]) -> i32 {
    // Each num in target is unique
    // Map them to their indices
    // Thus their indices should form an increasing subseq in arr
    // LCS => LIS
    let indices = target
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, &num)| {
            acc.insert(num, i);
            acc
        });
    let mut lis = vec![];
    for num in arr.iter() {
        let Some(&idx) = indices.get(num) else {
            continue; // ignore num not in target
        };
        let Err(pos) = lis.binary_search(&idx) else {
            continue;
        };
        if let Some(v) = lis.get_mut(pos) {
            *v = idx;
        } else {
            lis.push(idx);
        }
    }
    (target.len() - lis.len()) as _
    // let mut stack = BTreeSet::new();
    // for &num in arr.iter() {
    //     let Some(&idx) = indices.get(&num) else {
    //         continue; // ignore num not in target
    //     };
    //     // If current idx is bigger than all in stack
    //     if stack.is_empty() || stack.last().is_some_and(|&v| v < idx) {
    //         // if stack.last().is_none_or(|&v| v < idx) {
    //         stack.insert(idx);
    //         continue;
    //     }
    //     // else replace the closest bigger value
    //     let key = *stack.range(idx..).next().unwrap();
    //     stack.remove(&key);
    //     stack.insert(idx);
    // }
    // (target.len() - stack.len()) as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(min_operations(&[5, 1, 3], &[9, 4, 2, 3, 4]), 2);
        assert_eq!(
            min_operations(&[6, 4, 8, 1, 3, 2], &[4, 7, 6, 2, 3, 8, 6, 1]),
            3
        );
    }

    #[test]
    fn test() {}
}
