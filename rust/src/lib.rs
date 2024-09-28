mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use rand::Rng;

#[derive(Debug, Clone)]
struct Solution {
    prefix: Vec<i32>,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        prefix.push(w[0]);
        for (i, &num) in w.iter().enumerate().skip(1) {
            prefix.push(prefix[i - 1] + num);
        }
        Self { prefix }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        // 0 is unreachable while last value is
        // using 0..last tilted the distribution left
        let num = rng.gen_range(1..=*self.prefix.last().unwrap());
        self.prefix
            .binary_search(&num)
            .unwrap_or_else(std::convert::identity) as _
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
