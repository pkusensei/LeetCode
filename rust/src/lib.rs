mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone)]
struct Solution {
    rng: ThreadRng,
    range: i32,
    bmap: HashMap<i32, i32>,
}

impl Solution {
    fn new(n: i32, mut black: Vec<i32>) -> Self {
        let set: HashSet<_> = black.iter().copied().collect();
        let mut map = HashMap::new();
        let range = n - set.len() as i32;
        let mut idx = 0;
        black.sort_unstable();
        for num in range..n {
            if !set.contains(&num) {
                map.insert(black[idx], num);
                idx += 1;
            }
        }
        Self {
            rng: rand::thread_rng(),
            range,
            bmap: map,
        }
    }

    fn pick(&mut self) -> i32 {
        let i = self.rng.gen_range(0..self.range);
        if let Some(&n) = self.bmap.get(&i) {
            return n;
        } else {
            i
        }
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
