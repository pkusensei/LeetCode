mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn get_number_of_backlog_orders(orders: &[[i32; 3]]) -> i32 {
    let mut sell_heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
    let mut buy_heap = BinaryHeap::new();
    for o in orders.iter() {
        let [price, mut amount, t] = o[..] else {
            unreachable!()
        };
        if t == 0 {
            // buy
            while amount > 0
                && sell_heap
                    .peek()
                    .is_some_and(|&(Reverse(p), _count)| p <= price)
            {
                let (Reverse(p), count) = sell_heap.pop().unwrap();
                if count > amount {
                    sell_heap.push((Reverse(p), count - amount));
                    amount = 0;
                    break;
                } else {
                    amount -= count;
                }
            }
            if amount > 0 {
                buy_heap.push((price, amount));
            }
        } else {
            // sell
            while amount > 0 && buy_heap.peek().is_some_and(|&(p, _count)| p >= price) {
                let (p, count) = buy_heap.pop().unwrap();
                if count > amount {
                    buy_heap.push((p, count - amount));
                    amount = 0;
                    break;
                } else {
                    amount -= count
                }
            }
            if amount > 0 {
                sell_heap.push((Reverse(price), amount));
            }
        }
    }
    sell_heap
        .into_iter()
        .map(|(_, c)| c)
        .chain(buy_heap.into_iter().map(|(_, c)| c))
        .fold(0, |acc, v| (acc + v) % 1_000_000_007)
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
            get_number_of_backlog_orders(&[[10, 5, 0], [15, 2, 1], [25, 1, 1], [30, 4, 0]]),
            6
        );
        assert_eq!(
            get_number_of_backlog_orders(&[
                [7, 1000000000, 1],
                [15, 3, 0],
                [5, 999999995, 0],
                [5, 1, 1]
            ]),
            999999984
        );
    }

    #[test]
    fn test() {}
}
