mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::{BinaryHeap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn min_time(n: i32, k: i32, _m: i32, time: &[i32], mul: &[f64]) -> f64 {
    if n > 1 && k == 1 {
        return -1.0;
    }
    let start = State::new(n);
    let mut heap = BinaryHeap::from([start]);
    let mut map = HashMap::from([(start.as_key(), start.time)]);
    while let Some(s) = heap.pop() {
        if !s.forward && s.mask == (1 << n) - 1 {
            return s.time;
        }
        if map.get(&s.as_key()).is_some_and(|&v| v < s.time) {
            continue;
        }
        for next in s.cross(k as u32, time, mul) {
            if map.get(&next.as_key()).is_none_or(|&v| v > next.time) {
                map.insert(next.as_key(), next.time);
                heap.push(next);
            }
        }
    }
    -1.0
}

#[derive(Clone, Copy, PartialEq)]
struct State {
    mask: i16,
    time: f64,
    stage: usize,
    forward: bool,
}

impl State {
    const fn new(n: i32) -> Self {
        Self {
            mask: (1 << n) - 1,
            time: 0.0,
            stage: 0,
            forward: true,
        }
    }

    fn cross(self, k: u32, time: &[i32], mul: &[f64]) -> Vec<Self> {
        if self.forward {
            self.cross_forward(k, time, mul)
        } else {
            self.cross_back(time, mul)
        }
    }

    fn cross_forward(self, k: u32, time: &[i32], mul: &[f64]) -> Vec<State> {
        let n = time.len();
        let m = mul.len();
        let mut res = vec![];
        for pick in 1..1 << n {
            if pick & self.mask == pick && pick.count_ones() <= k {
                let t = (0..n)
                    .filter_map(|bit| {
                        if (pick >> bit) & 1 == 1 {
                            Some(time[bit])
                        } else {
                            None
                        }
                    })
                    .max()
                    .map(|t| f64::from(t) * mul[self.stage])
                    .unwrap_or(100_000_000.0);
                let stage = (t.floor() as usize + self.stage) % m;
                let mask = (((1 << n) - 1) ^ self.mask) | pick;
                res.push(Self {
                    mask,
                    time: self.time + t,
                    stage,
                    forward: false,
                });
            }
        }
        res
    }

    fn cross_back(self, time: &[i32], mul: &[f64]) -> Vec<Self> {
        let n = time.len();
        let m = mul.len();
        let mut res = vec![];
        for bit in 0..n {
            if (self.mask >> bit) & 1 == 1 {
                let t = f64::from(time[bit]) * mul[self.stage];
                let stage = (t.floor() as usize + self.stage) % m;
                res.push(Self {
                    mask: (((1 << n) - 1) ^ self.mask) | (1 << bit),
                    time: self.time + t,
                    stage,
                    forward: true,
                });
            }
        }
        res
    }

    const fn as_key(self) -> (i16, usize, bool) {
        (self.mask, self.stage, self.forward)
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .time
            .total_cmp(&self.time)
            .then(other.mask.count_ones().cmp(&self.mask.count_ones()))
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(min_time(1, 1, 2, &[5], &[1.0, 1.3]), 5.0);
        assert_eq!(min_time(3, 2, 3, &[2, 5, 8], &[1.0, 1.5, 0.75]), 14.5);
        assert_eq!(min_time(2, 1, 2, &[10, 10], &[2.0, 2.0]), -1.0);
    }

    #[test]
    fn test() {}
}
