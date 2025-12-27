mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn most_booked(n: i32, mut meetings: Vec<[i32; 2]>) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = n as usize;
    let mut freq = vec![0; n];
    meetings.sort_unstable();
    let mut free: BinaryHeap<_> = (0..n).map(Reverse).collect();
    let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    for m in meetings {
        let [start, end] = m[..] else { unreachable!() };
        while let Some(&Reverse((end, i))) = busy.peek()
            && end <= i64::from(start)
        {
            busy.pop();
            free.push(Reverse(i));
        }
        if let Some(Reverse(i)) = free.pop() {
            freq[i] += 1;
            busy.push(Reverse((i64::from(end), i)));
        } else if let Some(Reverse((prev, i))) = busy.pop() {
            freq[i] += 1;
            busy.push(Reverse((prev + i64::from(end - start), i)));
        }
    }
    let mut res = 0;
    let mut maxf = freq[0];
    for (i, &f) in freq.iter().enumerate() {
        if f > maxf {
            maxf = f;
            res = i;
        }
    }
    res as i32
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
            most_booked(4, vec![[18, 19], [3, 12], [17, 19], [2, 13], [7, 10]]),
            0
        );
    }
}
