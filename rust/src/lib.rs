mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct RangeFreqQuery {
    data: HashMap<i32, Vec<i32>>,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        Self {
            data: (0..).zip(arr).fold(HashMap::new(), |mut acc, (i, num)| {
                acc.entry(num).or_default().push(i);
                acc
            }),
        }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let Some(v) = self.data.get(&value) else {
            return 0;
        };
        let a = v.partition_point(|&i| i < left);
        let b = v.partition_point(|&i| i <= right);
        (b - a) as i32
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
        let rf = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
        assert_eq!(rf.query(1, 2, 4), 1); // return 1. The value 4 occurs 1 time in the subarray [33, 4]
        assert_eq!(rf.query(0, 11, 33), 2); // return 2. The value 33 occurs 2 times in the whole array.
    }

    #[test]
    fn test() {}
}
