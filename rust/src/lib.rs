mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct MajorityChecker {
    arr: Vec<i32>,
    map: HashMap<i32, Vec<usize>>,
    buckets: Vec<Option<i32>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let map = arr
            .iter()
            .enumerate()
            .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &v)| {
                acc.entry(v).or_default().push(i);
                acc
            });
        let n = arr.len();
        let b = n.isqrt();
        let buckets = arr.chunks(b).map(vote).collect();
        Self { arr, map, buckets }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let [left, right] = [left, right].map(|v| v as usize);
        let b = self.arr.len().isqrt();
        let [buc_left, buc_right] = [left, right].map(|v| v / b);
        if buc_left == buc_right {
            // same bucket
            if let Some(v) = vote(&self.arr[left..=right])
                && self.count(v, left, right) >= threshold
            {
                return v;
            }
            return -1;
        }
        // left partial
        let rr = (1 + buc_left) * b - 1;
        if let Some(v) = vote(&self.arr[left..=rr])
            && self.count(v, left, right) >= threshold
        {
            return v;
        }
        // right partial
        let ll = b * buc_right;
        if let Some(v) = vote(&self.arr[ll..=right])
            && self.count(v, left, right) >= threshold
        {
            return v;
        }
        for i in 1 + buc_left..buc_right {
            if let Some(v) = self.buckets[i]
                && self.count(v, left, right) >= threshold
            {
                return v;
            }
        }
        -1
    }

    fn count(&self, num: i32, left: usize, right: usize) -> i32 {
        let Some(arr) = self.map.get(&num) else {
            return 0;
        };
        let a = arr.partition_point(|&v| v < left);
        let b = arr.partition_point(|&v| v <= right);
        (b - a) as i32
    }
}

fn vote(nums: &[i32]) -> Option<i32> {
    let mut counter = 0;
    let mut major = 0;
    for &num in nums.iter() {
        if counter == 0 {
            major = num;
            counter = 0;
        }
        if num == major {
            counter += 1;
        } else {
            counter -= 1;
        }
    }
    let freq = nums.iter().filter(|&&v| v == major).count();
    if 2 * freq > nums.len() {
        Some(major)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
    fn test() {
        let m = MajorityChecker::new(vec![2, 2, 1, 2, 1, 2, 2, 1, 1, 2]);
        assert_eq!(-1, m.query(0, 5, 6));
    }
}
