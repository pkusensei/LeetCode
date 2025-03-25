mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_crossing_time(n: i32, _k: i32, time: &[[i32; 4]]) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut n = n as usize;
    let mut left: BinaryHeap<_> = time
        .iter()
        .enumerate()
        .map(|(id, t)| Worker {
            right: t[0],
            pick: t[1],
            left: t[2],
            put: t[3],
            id,
        })
        .collect();
    let mut right: BinaryHeap<Worker> = BinaryHeap::new();
    let mut left_queue = BinaryHeap::new();
    let mut right_queue = BinaryHeap::new();
    let mut bridge_end = 0;
    while n > 0 {
        while left_queue
            .peek()
            .is_some_and(|&(Reverse(v), _)| v <= bridge_end)
        {
            let (_, w) = left_queue.pop().unwrap();
            left.push(w);
        }
        while right_queue
            .peek()
            .is_some_and(|&(Reverse(v), _)| v <= bridge_end)
        {
            let (_, w) = right_queue.pop().unwrap();
            right.push(w);
        }
        if right_queue.len() + right.len() >= n {
            if let Some(w) = right.pop() {
                bridge_end += w.left;
                n -= 1;
            } else if let Some(v) = right_queue.peek().map(|&(Reverse(v), _)| v) {
                bridge_end = bridge_end.max(v);
            }
        } else {
            match [left.peek(), right.peek()] {
                [Some(_), None] => {
                    let w = left.pop().unwrap();
                    bridge_end += w.right;
                    let end = bridge_end + w.pick;
                    right_queue.push((Reverse(end), w));
                }
                [None, Some(_)] | [Some(_), Some(_)] => {
                    let w = right.pop().unwrap();
                    bridge_end += w.left;
                    let end = bridge_end + w.put;
                    left_queue.push((Reverse(end), w));
                    n -= 1;
                }
                [None, None] => {
                    let mut temp = i32::MAX;
                    if let Some(v) = left_queue.peek().map(|&(Reverse(v), _)| v) {
                        temp = temp.min(v);
                    }
                    if let Some(v) = right_queue.peek().map(|&(Reverse(v), _)| v) {
                        temp = temp.min(v);
                    }
                    bridge_end = bridge_end.max(temp);
                }
            }
        }
    }
    bridge_end
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Worker {
    right: i32,
    pick: i32,
    left: i32,
    put: i32,
    id: usize,
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.left + self.right)
            .cmp(&(other.left + other.right))
            .then(self.id.cmp(&other.id))
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
            find_crossing_time(1, 3, &[[1, 1, 2, 1], [1, 1, 3, 1], [1, 1, 4, 1]]),
            6
        );
        assert_eq!(
            find_crossing_time(3, 2, &[[1, 5, 1, 8], [10, 10, 10, 10]]),
            37
        );
    }

    #[test]
    fn test() {}
}
