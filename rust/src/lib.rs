mod dsu;
mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn min_interval(intervals: &mut [[i32; 2]], queries: &[i32]) -> Vec<i32> {
    intervals.sort_unstable();
    let mut qis: Vec<_> = queries.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    qis.sort_unstable_by_key(|(_, v)| *v);
    let mut res = vec![-1; queries.len()];
    let mut heap = BinaryHeap::new();
    let mut idx = 0;
    for &(i, q) in qis.iter() {
        while intervals.get(idx).is_some_and(|v| v[0] <= q) {
            heap.push(Chunk {
                start: intervals[idx][0],
                end: intervals[idx][1],
            });
            idx += 1;
        }
        while heap.peek().is_some_and(|c| c.end < q) {
            heap.pop();
        }
        if let Some(c) = heap.peek() {
            res[i] = c.end + 1 - c.start;
        }
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Chunk {
    start: i32,
    end: i32,
}

impl PartialOrd for Chunk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Chunk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.end - other.start)
            .cmp(&(self.end - self.start))
            .then(other.end.cmp(&self.end))
    }
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
        assert_eq!(
            min_interval(&mut [[1, 4], [2, 4], [3, 6], [4, 4]], &mut [2, 3, 4, 5]),
            [3, 3, 1, 4]
        );
        assert_eq!(
            min_interval(&mut [[2, 3], [2, 5], [1, 8], [20, 25]], &mut [2, 19, 5, 22]),
            [2, -1, 4, 6]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            min_interval(
                &mut [[4, 5], [5, 8], [1, 9], [8, 10], [1, 6]],
                &mut [7, 9, 3, 9, 3]
            ),
            [4, 3, 6, 3, 6]
        );
    }
}
