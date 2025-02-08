mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn get_order(tasks: &[[i32; 2]]) -> Vec<i32> {
    let n = tasks.len();
    let mut sorted: Vec<_> = tasks
        .iter()
        .enumerate()
        .map(|(i, t)| (i, t[0], t[1]))
        .collect();
    sorted.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)));
    let mut idx = 0;
    let mut heap = BinaryHeap::from([Reverse((sorted[idx].2, sorted[idx].0))]);
    let mut start = sorted[idx].1;
    idx += 1;
    let mut res = Vec::with_capacity(n);
    while !heap.is_empty() || idx < n {
        if let Some(Reverse((dur, i))) = heap.pop() {
            res.push(i as i32);
            start += dur;
            while sorted.get(idx).is_some_and(|v| v.1 <= start) {
                heap.push(Reverse((sorted[idx].2, sorted[idx].0)));
                idx += 1;
            }
        } else if idx < n {
            start = sorted[idx].1;
            heap.push(Reverse((sorted[idx].2, sorted[idx].0)));
            idx += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {

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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
    fn basics() {
        assert_eq!(get_order(&[[1, 2], [2, 4], [3, 2], [4, 1]]), [0, 2, 3, 1]);
        assert_eq!(
            get_order(&[[7, 10], [7, 12], [7, 5], [7, 4], [7, 2]]),
            [4, 3, 2, 0, 1]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            get_order(&[
                [19, 13],
                [16, 9],
                [21, 10],
                [32, 25],
                [37, 4],
                [49, 24],
                [2, 15],
                [38, 41],
                [37, 34],
                [33, 6],
                [45, 4],
                [18, 18],
                [46, 39],
                [12, 24]
            ]),
            [6, 1, 2, 9, 4, 10, 0, 11, 5, 13, 3, 8, 12, 7]
        );
    }
}
