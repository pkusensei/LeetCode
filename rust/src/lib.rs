mod helper;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        if self.small.is_empty() {
            // empty
            self.small.push(num);
        } else {
            let mid = self.small.peek().copied().unwrap();
            if num < mid {
                self.small.push(num);
            } else {
                self.large.push(Reverse(num));
            }
        }
        // Make sure that
        // When there's an odd number of elements
        // small.len() == 1+large.len()
        // While for even amount
        // small.len() == large.len()
        if self.small.len() > self.large.len() + 1 {
            let v = self.small.pop().unwrap();
            self.large.push(Reverse(v));
        }
        if self.small.len() < self.large.len() {
            let v = self.large.pop().unwrap().0;
            self.small.push(v);
        }
    }

    fn find_median(&self) -> f64 {
        let a = self.small.peek().copied();
        if self.small.len() > self.large.len() {
            a.unwrap_or(0).into()
        } else {
            let b = self.large.peek().map(|r| r.0);
            f64::from(a.unwrap_or(0) + b.unwrap_or(0)) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut f = MedianFinder::new();
        f.add_num(1); // arr = [1]
        f.add_num(2); // arr = [1, 2]
        debug_assert_eq!(f.find_median(), 1.5); // return 1.5 (i.e., (1 + 2) / 2)
        f.add_num(3); // arr[1, 2, 3]
        debug_assert_eq!(f.find_median(), 2.0); // return 2.0
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
