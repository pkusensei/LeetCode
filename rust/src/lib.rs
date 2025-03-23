mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct Allocator {
    slots: Vec<bool>,
    map: HashMap<i32, Vec<[usize; 2]>>,
}

impl Allocator {
    fn new(n: i32) -> Self {
        Self {
            slots: vec![false; n as usize],
            map: HashMap::new(),
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let n = size as usize;
        let Some(start) = self
            .slots
            .windows(n)
            .enumerate()
            .find_map(|(i, w)| if w.iter().all(|&v| !v) { Some(i) } else { None })
        else {
            return -1;
        };
        self.slots[start..start + n].fill(true);
        self.map.entry(m_id).or_default().push([start, start + n]);
        start as i32
    }

    fn free_memory(&mut self, m_id: i32) -> i32 {
        let Some(v) = self.map.remove(&m_id) else {
            return 0;
        };
        let mut res = 0;
        for [a, b] in v {
            self.slots[a..b].fill(false);
            res += b - a;
        }
        res as _
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
    fn basics() {}

    #[test]
    fn test() {}
}
