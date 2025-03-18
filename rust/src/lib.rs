mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn most_booked(n: i32, meetings: &mut [[i32; 2]]) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = n as usize;
    meetings.sort_unstable();
    let mut count = vec![0; n];
    // min heap by id
    let mut free: BinaryHeap<_> = (0..n).map(|i| Reverse(i)).collect();
    // min heap by endtime
    let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    for meet in meetings.iter() {
        let [start, end] = [0, 1].map(|i| i64::from(meet[i]));
        while busy.peek().is_some_and(|Reverse((time, _))| *time <= start) {
            let Reverse((_, id)) = busy.pop().unwrap();
            free.push(Reverse(id));
        }
        if let Some(Reverse(id)) = free.pop() {
            count[id] += 1;
            busy.push(Reverse((i64::from(end), id)));
        } else {
            let Reverse((prev, id)) = busy.pop().unwrap();
            count[id] += 1;
            busy.push(Reverse((prev + i64::from(end - start), id)));
        }
    }
    let mut res = 0;
    let mut max = 0;
    for (i, &c) in count.iter().enumerate() {
        if c > max {
            res = i as i32;
            max = c;
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
        assert_eq!(most_booked(2, &mut [[0, 10], [1, 5], [2, 7], [3, 4]]), 0);
        assert_eq!(
            most_booked(3, &mut [[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]),
            1
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            most_booked(4, &mut [[18, 19], [3, 12], [17, 19], [2, 13], [7, 10]]),
            0
        );
    }
}
