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
struct RangeModule {
    data: BTreeMap<i32, i32>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, mut left: i32, mut right: i32) {
        if let Some(p) = self.data.range(..=left).next_back()
            && left <= *p.1
        {
            left = *p.0;
        }
        if let Some(p) = self.data.range(..=right).next_back()
            && right < *p.1
        {
            right = *p.1;
        }
        let del: Vec<_> = self.data.range(left..right).map(|p| *p.0).collect();
        for k in del {
            self.data.remove(&k);
        }
        self.data.insert(left, right);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.data
            .range(..=left)
            .next_back()
            .is_some_and(|p| right <= *p.1)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        if let Some(p) = self.data.range(..=right).next_back()
            && right < *p.1
        {
            let val = *p.1;
            self.data.insert(right, val);
        }
        if let Some(p) = self.data.range(..=left).next_back()
            && left < *p.1
        {
            let key = *p.0;
            self.data.insert(key, left);
        }
        let del: Vec<_> = self.data.range(left..right).map(|p| *p.0).collect();
        for k in del {
            self.data.remove(&k);
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
    fn basics() {
        let mut rm = RangeModule::new();
        rm.add_range(10, 20);
        rm.remove_range(14, 16);
        assert!(rm.query_range(10, 14)); // return True,(Every number in [10, 14) is being tracked)
        assert!(!rm.query_range(13, 15)); // return False,(Numbers like 14, 14.03, 14.17 in [13, 15) are not being tracked)
        assert!(rm.query_range(16, 17));
    }

    #[test]
    fn test() {}
}
