mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
#[derive(Default)]
struct EventManager {
    map: HashMap<i32, i32>,
    heap: BinaryHeap<(i32, Reverse<i32>)>,
}

impl EventManager {
    fn new(events: Vec<Vec<i32>>) -> Self {
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::new();
        for e in events {
            let [id, p] = e[..] else { unreachable!() };
            map.insert(id, p);
            heap.push((p, Reverse(id)));
        }
        Self { map, heap }
    }

    fn update_priority(&mut self, event_id: i32, new_priority: i32) {
        *self.map.entry(event_id).or_insert(new_priority) = new_priority;
        self.heap.push((new_priority, Reverse(event_id)));
    }

    fn poll_highest(&mut self) -> i32 {
        while let Some(&(p, Reverse(id))) = self.heap.peek()
            && self.map.get(&id).is_none_or(|&v| v != p)
        {
            self.heap.pop();
        }
        let Some((_, Reverse(id))) = self.heap.pop() else {
            return -1;
        };
        self.map.remove(&id);
        id
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
