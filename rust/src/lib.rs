mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_cost(costs: &[i32], k: i32, candidates: i32) -> i64 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = costs.len();
    let c = candidates as usize;
    let mut left_heap: BinaryHeap<_> = costs[..c].iter().copied().map(Reverse).collect();
    let mut right_heap: BinaryHeap<_> = costs[(n - c).max(c)..]
        .iter()
        .copied()
        .map(Reverse)
        .collect();
    let mut res = 0;
    let mut left = c;
    let mut right = (n - c).max(c) - 1;
    for _ in 0..k {
        match [left_heap.peek(), right_heap.peek()] {
            [Some(&Reverse(a)), Some(&Reverse(b))] if a <= b => {
                left_heap.pop();
                res += i64::from(a);
                if left <= right {
                    left_heap.push(Reverse(costs[left]));
                    left += 1;
                }
            }
            [Some(&Reverse(_a)), Some(&Reverse(b))] => {
                right_heap.pop();
                res += i64::from(b);
                if left <= right {
                    right_heap.push(Reverse(costs[right]));
                    right -= 1;
                }
            }
            [Some(&Reverse(a)), None] => {
                left_heap.pop();
                res += i64::from(a);
            }
            [None, Some(&Reverse(b))] => {
                right_heap.pop();
                res += i64::from(b);
            }
            _ => break,
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
        assert_eq!(total_cost(&[17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4), 11);
        assert_eq!(total_cost(&[1, 2, 4, 1], 3, 3), 4);
    }

    #[test]
    fn test() {}
}
