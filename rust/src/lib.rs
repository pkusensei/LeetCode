mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::BTreeMap;

struct MyCalendar {
    data: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some(kv) = self.data.range(..end_time).next_back()
            && *kv.1 > start_time
        {
            return false;
        }
        self.data.insert(start_time, end_time).is_none()
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
