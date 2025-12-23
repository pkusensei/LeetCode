mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_two_events(mut events: Vec<[i32; 3]>) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};

    events.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
    let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
    let mut res = 0;
    let mut prev_max = 0;
    for e in events {
        let [start, end, val] = e[..] else {
            unreachable!()
        };
        while let Some((Reverse(prev_e), prev_v)) = heap.peek()
            && *prev_e < start
        {
            prev_max = prev_max.max(*prev_v);
            heap.pop();
        }
        res = res.max(prev_max + val);
        heap.push((Reverse(end), val));
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
        assert_eq!(
            max_two_events(vec![
                [28, 81, 48],
                [27, 90, 94],
                [97, 99, 79],
                [5, 35, 81],
                [65, 94, 84],
                [65, 83, 58],
                [94, 94, 31],
                [39, 52, 73]
            ]),
            173
        );
        assert_eq!(
            max_two_events(vec![
                [22, 44, 9],
                [93, 96, 48],
                [56, 90, 3],
                [80, 92, 45],
                [63, 73, 69],
                [73, 96, 33],
                [11, 23, 84],
                [59, 72, 29],
                [89, 100, 46]
            ]),
            153
        );
    }
}
