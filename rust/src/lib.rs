mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, HashMap};

#[allow(unused_imports)]
use helper::*;

struct TopVotedCandidate {
    data: BTreeMap<i32, i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut tally = HashMap::new();
        let mut data = BTreeMap::new();
        for (idx, (&p, t)) in persons.iter().zip(times).enumerate() {
            *tally.entry(p).or_insert(0) += 1;
            let max = *tally.values().max().unwrap_or(&1);
            if let Some(&v) = persons[..=idx].iter().rev().find(|&v| tally[&v] == max) {
                data.insert(t, v);
            }
        }
        Self { data }
    }

    fn q(&self, t: i32) -> i32 {
        self.data
            .range(..=t)
            .next_back()
            .map(|(_, &p)| p)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let v = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
        debug_assert_eq!(v.q(3), 0); // return 0, At time 3, the votes are [0], and 0 is leading.
        debug_assert_eq!(v.q(12), 1); // return 1, At time 12, the votes are [0,1,1], and 1 is leading.
        debug_assert_eq!(v.q(25), 1); // return 1, At time 25, the votes are [0,1,1,0,0,1], and 1 is leading (as ties go to the most recent vote.)
        debug_assert_eq!(v.q(15), 0); // return 0
        debug_assert_eq!(v.q(24), 0); // return 0
        debug_assert_eq!(v.q(8), 1); // return 1
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
