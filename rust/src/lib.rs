mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn eaten_apples(apples: &[i32], days: &[i32]) -> i32 {
    let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
    let mut res = 0;
    for (i, (&count, &day)) in (0..).zip(apples.iter().zip(days.iter())) {
        if count > 0 {
            heap.push((Reverse(i + day), count));
        }
        while heap.peek().is_some_and(|&(Reverse(d), _)| d <= i) {
            heap.pop();
        }
        if let Some((Reverse(d), c)) = heap.pop() {
            res += 1;
            if c > 1 {
                heap.push((Reverse(d), c - 1));
            }
        } else {
            continue;
        };
    }
    let mut curr = apples.len() as i32;
    while !heap.is_empty() {
        while heap.peek().is_some_and(|&(Reverse(d), _)| d <= curr) {
            heap.pop();
        }
        let Some((Reverse(d), c)) = heap.pop() else {
            break;
        };
        let delta = (d - curr).min(c);
        res += delta;
        curr += delta;
    }
    res
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
        assert_eq!(eaten_apples(&[1, 2, 3, 5, 2], &[3, 2, 1, 4, 2]), 7);
        assert_eq!(eaten_apples(&[3, 0, 0, 0, 0, 2], &[3, 0, 0, 0, 0, 2]), 5);
    }

    #[test]
    fn test() {}
}
