mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct MKAverage {
    m: usize,
    k: i32,
    sum: i32,
    queue: VecDeque<i32>,
    map: BTreeMap<i32, i32>,
}

impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        Self {
            m: m as usize,
            k,
            ..Default::default()
        }
    }

    fn add_element(&mut self, num: i32) {
        self.queue.push_back(num);
        *self.map.entry(num).or_insert(0) += 1;
        self.sum += num;
        if self.queue.len() > self.m {
            let first = self.queue.pop_front().unwrap();
            self.sum -= first;
            self.map.entry(first).and_modify(|v| *v -= 1);
            if self.map[&first] == 0 {
                self.map.remove(&first);
            }
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.queue.len() < self.m {
            return -1;
        }
        let mut sum = self.sum;
        let mut k = self.k;
        for (&num, &count) in self.map.iter() {
            let del = k.min(count);
            sum -= del * num;
            k -= del;
            if k == 0 {
                break;
            }
        }
        k = self.k;
        for (&num, &count) in self.map.iter().rev() {
            let del = k.min(count);
            sum -= del * num;
            k -= del;
            if k == 0 {
                break;
            }
        }
        (f64::from(sum) / (self.m as f64 - f64::from(2 * self.k))).floor() as _
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
    fn basics() {
        let mut obj = MKAverage::new(3, 1);
        obj.add_element(3); // current elements are [3]
        obj.add_element(1); // current elements are [3,1]
        assert_eq!(obj.calculate_mk_average(), -1); // return -1, because m = 3 and only 2 elements exist.
        obj.add_element(10); // current elements are [3,1,10]
        assert_eq!(obj.calculate_mk_average(), 3); // The last 3 elements are [3,1,10].
                                                   // After removing smallest and largest 1 element the container will be [3].
                                                   // The average of [3] equals 3/1 = 3, return 3
        obj.add_element(5); // current elements are [3,1,10,5]
        obj.add_element(5); // current elements are [3,1,10,5,5]
        obj.add_element(5); // current elements are [3,1,10,5,5,5]
        assert_eq!(obj.calculate_mk_average(), 5); // The last 3 elements are [5,5,5].
                                                   // After removing smallest and largest 1 element the container will be [5].
                                                   // The average of [5] equals 5/1 = 5, return 5
    }

    #[test]
    fn test() {}
}
