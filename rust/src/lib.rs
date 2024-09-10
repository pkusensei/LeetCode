mod helper;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct SummaryRanges {
    data: BTreeSet<[i32; 2]>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            data: BTreeSet::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        let mut range = [value; 2];
        let left = self.data.range(..range).last().copied().unwrap_or(range);
        let right = self.data.range(range..).next().copied().unwrap_or(range);
        if left[1] + 1 >= value {
            range = [left[0], value.max(left[1])];
            self.data.remove(&left);
        }
        if right[0] - 1 <= value && range[1] < right[1] {
            range = [range[0].min(right[0]), right[1]];
            self.data.remove(&right);
        }
        self.data.insert(range);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.data.iter().map(|d| d.to_vec()).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut range = SummaryRanges::new();
        range.add_num(1); // arr = [1]
        debug_assert_eq!(range.get_intervals(), [[1, 1]]);
        range.add_num(3); // arr = [1, 3]
        debug_assert_eq!(range.get_intervals(), [[1, 1], [3, 3]]);
        range.add_num(7); // arr = [1, 3, 7]
        debug_assert_eq!(range.get_intervals(), [[1, 1], [3, 3], [7, 7]]);
        range.add_num(2); // arr = [1, 2, 3, 7]
        debug_assert_eq!(range.get_intervals(), [[1, 3], [7, 7]]);
        range.add_num(6); // arr = [1, 2, 3, 6, 7]
        debug_assert_eq!(range.get_intervals(), [[1, 3], [6, 7]]);
    }

    #[test]
    fn test() {
        let mut range = SummaryRanges::new();
        range.add_num(6);
        debug_assert_eq!(range.get_intervals(), [[6, 6]]);
        range.add_num(6);
        debug_assert_eq!(range.get_intervals(), [[6, 6]]);
        range.add_num(0);
        debug_assert_eq!(range.get_intervals(), [[0, 0], [6, 6]]);
        range.add_num(4);
        debug_assert_eq!(range.get_intervals(), [[0, 0], [4, 4], [6, 6]]);
        range.add_num(8);
        debug_assert_eq!(range.get_intervals(), [[0, 0], [4, 4], [6, 6], [8, 8]]);
        range.add_num(7);
        debug_assert_eq!(range.get_intervals(), [[0, 0], [4, 4], [6, 8]]);
        range.add_num(6);
        debug_assert_eq!(range.get_intervals(), [[0, 0], [4, 4], [6, 8]]);
        range.add_num(4);
        debug_assert_eq!(range.get_intervals(), [[0, 0], [4, 4], [6, 8]]);
        range.add_num(7);
        debug_assert_eq!(range.get_intervals(), [[0, 0], [4, 4], [6, 8]]);
        range.add_num(5);
        debug_assert_eq!(range.get_intervals(), [[0, 0], [4, 8]]);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
