mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::BTreeMap;
struct SummaryRanges {
    map: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        if let Some((&left_start, &left_end)) = self.map.range(..=value).next_back()
            && left_end + 1 >= value
        {
            if let Some(end) = self.map.remove(&(1 + value)) {
                self.map.insert(left_start, end);
            } else {
                self.map.insert(left_start, left_end.max(value));
            }
        } else if let Some(end) = self.map.remove(&(1 + value)) {
            self.map.insert(value, end);
        } else {
            self.map.insert(value, value);
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.map.iter().map(|(k, v)| vec![*k, *v]).collect()
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
        let mut sr = SummaryRanges::new();
        sr.add_num(6);
        sr.add_num(8);
        assert_eq!(sr.get_intervals(), [[6, 6], [8, 8]]);
        sr.add_num(7);
        assert_eq!(sr.get_intervals(), [[6, 8]]);
        sr.add_num(6);
        assert_eq!(sr.get_intervals(), [[6, 8]]);
    }

    #[test]
    fn test() {}
}
