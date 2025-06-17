mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{HashMap, HashSet, VecDeque};

struct Router {
    set: HashSet<Packet>,
    queue: VecDeque<Packet>,
    dst_times: HashMap<i32, VecDeque<Packet>>,
    n: usize,
}

impl Router {
    fn new(memoryLimit: i32) -> Self {
        let n = memoryLimit as usize;
        Self {
            set: HashSet::with_capacity(n),
            queue: VecDeque::with_capacity(n),
            dst_times: HashMap::with_capacity(n),
            n,
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let p = Packet {
            time: timestamp,
            src: source,
            dst: destination,
        };
        if self.set.contains(&p) {
            return false;
        }
        if self.queue.len() == self.n {
            _ = self.forward_packet();
        }
        self.dst_times.entry(destination).or_default().push_back(p);
        self.queue.push_back(p);
        self.set.insert(p)
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        let Some(p) = self.queue.pop_front() else {
            return vec![];
        };
        self.set.remove(&p);
        let packets = self.dst_times.entry(p.dst).or_default();
        if packets.front().is_some_and(|&v| v == p) {
            packets.pop_front();
        }
        vec![p.src, p.dst, p.time]
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let Some(packets) = self.dst_times.get(&destination) else {
            return 0;
        };
        let i1 = packets.partition_point(|p| p.time < start_time);
        let i2 = packets.partition_point(|p| p.time <= end_time);
        (i2 - i1) as i32
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Packet {
    time: i32,
    src: i32,
    dst: i32,
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
