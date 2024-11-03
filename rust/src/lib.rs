mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct StockSpanner {
    stack: Vec<(i32, i32)>, // (count, num)
}

impl StockSpanner {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut res = 1;
        while self.stack.last().is_some_and(|&(_, v)| v <= price) {
            let Some((count, _)) = self.stack.pop() else {
                return 1;
            };
            res += count;
        }
        self.stack.push((res, price));
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut it = StockSpanner::new();
        debug_assert_eq!(it.next(100), 1); // return 1
        debug_assert_eq!(it.next(80), 1); // return 1
        debug_assert_eq!(it.next(60), 1); // return 1
        debug_assert_eq!(it.next(70), 2); // return 2
        debug_assert_eq!(it.next(60), 1); // return 1
        debug_assert_eq!(it.next(75), 4); // return 4, because the last 4 prices (including today's price of 75) were less than or equal to today's price.
        debug_assert_eq!(it.next(85), 6); // return 6
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
