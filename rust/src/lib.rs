mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit(prices: &[i32], fee: i32) -> i32 {
    // Greedy; for every num there are two states
    // Do something => buy or sell
    // Hold => do nothing
    let mut buy = -prices[0];
    let mut sell = 0;
    for &num in prices.iter().skip(1) {
        let temp = buy;
        buy = buy.max(sell - num);
        sell = sell.max(temp + num - fee);
    }
    sell.max(buy)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_profit(&[1, 3, 2, 8, 4, 9], 2), 8);
        debug_assert_eq!(max_profit(&[1, 3, 7, 5, 10, 3], 3), 6);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
