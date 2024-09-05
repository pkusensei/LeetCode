mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit(prices: &[i32]) -> i32 {
    let (mut buy, mut sell, mut cool) = (i32::MIN, 0, 0);
    for &num in prices.iter() {
        let b = buy.max(cool - num);
        let s = sell.max(buy + num);
        let c = sell.max(cool);
        (buy, sell, cool) = (b, s, c)
    }
    sell
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_profit(&[1, 2, 3, 0, 2]), 3);
        debug_assert_eq!(max_profit(&[1]), 0);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
