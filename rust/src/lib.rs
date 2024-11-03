mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, HashMap};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct FreqStack {
    num_freq: HashMap<i32, i32>,
    freq_num: BTreeMap<i32, Vec<i32>>,
}

impl FreqStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        if let Some(freq) = self.num_freq.get_mut(&val) {
            *freq += 1;
            self.freq_num.entry(*freq).or_default().push(val);
        } else {
            *self.num_freq.entry(val).or_insert(0) += 1;
            self.freq_num.entry(1).or_default().push(val);
        }
    }

    fn pop(&mut self) -> i32 {
        let Some((&k, v)) = self.freq_num.iter_mut().next_back() else {
            return 0;
        };
        let Some(res) = v.pop() else { return 0 };
        if v.is_empty() {
            self.freq_num.remove(&k);
        }
        if k > 1 {
            self.num_freq.insert(res, k - 1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut st = FreqStack::new();
        st.push(5); // The stack is [5]
        st.push(7); // The stack is [5,7]
        st.push(5); // The stack is [5,7,5]
        st.push(7); // The stack is [5,7,5,7]
        st.push(4); // The stack is [5,7,5,7,4]
        st.push(5); // The stack is [5,7,5,7,4,5]
        debug_assert_eq!(st.pop(), 5); // return 5, as 5 is the most frequent. The stack becomes [5,7,5,7,4].
        debug_assert_eq!(st.pop(), 7); // return 7, as 5 and 7 is the most frequent, but 7 is closest to the top. The stack becomes [5,7,5,4].
        debug_assert_eq!(st.pop(), 5); // return 5, as 5 is the most frequent. The stack becomes [5,7,4].
        debug_assert_eq!(st.pop(), 4); // return 4, as 4, 5 and 7 is the most frequent, but 4 is closest to the top. The stack becomes [5,7].
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
