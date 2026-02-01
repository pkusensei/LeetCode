mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::VecDeque;
#[derive(Default)]
struct RideSharingSystem {
    r_queue: VecDeque<i32>,
    d_queue: VecDeque<i32>,
    seen: Vec<bool>,
    enroute: Vec<bool>,
    cancel: Vec<bool>,
}

impl RideSharingSystem {
    fn new() -> Self {
        Self {
            seen: vec![false; 1001],
            enroute: vec![false; 1001],
            cancel: vec![false; 1001],
            ..Default::default()
        }
    }

    fn add_rider(&mut self, rider_id: i32) {
        self.seen[rider_id as usize] = true;
        self.r_queue.push_back(rider_id);
    }

    fn add_driver(&mut self, driver_id: i32) {
        self.d_queue.push_back(driver_id);
    }

    fn match_driver_with_rider(&mut self) -> Vec<i32> {
        while let Some(r) = self.r_queue.front()
            && self.cancel[*r as usize]
        {
            self.r_queue.pop_front();
        }
        if let Some(&r) = self.r_queue.front()
            && let Some(&d) = self.d_queue.front()
        {
            self.r_queue.pop_front();
            self.d_queue.pop_front();
            self.enroute[r as usize] = true;
            return vec![d, r];
        }
        vec![-1, -1]
    }

    fn cancel_rider(&mut self, rider_id: i32) {
        let id = rider_id as usize;
        if self.seen[id] && !self.enroute[id] {
            self.cancel[id] = true;
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}
