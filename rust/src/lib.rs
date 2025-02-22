mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct StockPrice {
    time_price: BTreeMap<i32, i32>,
    prices: BTreeMap<i32, i32>,
}

impl StockPrice {
    fn new() -> Self {
        Default::default()
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        *self.prices.entry(price).or_insert(0) += 1;
        if let Some(&v) = self.time_price.get(&timestamp) {
            let count = self.prices.entry(v).or_insert(0);
            *count -= 1;
            let prev = *count;
            if prev == 0 {
                self.prices.remove(&v);
            }
        }
        self.time_price.insert(timestamp, price);
    }

    fn current(&self) -> i32 {
        *self.time_price.values().last().unwrap()
    }

    fn maximum(&self) -> i32 {
        *self.prices.keys().last().unwrap()
    }

    fn minimum(&self) -> i32 {
        *self.prices.keys().next().unwrap()
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
    fn basics() {}

    #[test]
    fn test() {}
}
