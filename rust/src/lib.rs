mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i64> {
    use std::collections::{BTreeSet, HashMap};
    let [k, x] = [k, x].map(|v| v as usize);
    let mut res = vec![];
    let mut freq = HashMap::new();
    let mut big = BTreeSet::new();
    let mut small = BTreeSet::new();
    let mut sum = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let f = *freq.get(&num).unwrap_or(&0);
        let item = Item { f: 1 + f, num };
        if f > 0 {
            let a = Item { f, ..item };
            if !small.remove(&a) {
                big.remove(&a);
                sum -= a.sum();
            }
        }
        freq.insert(item.num, item.f);
        big.insert(item);
        sum += item.sum();
        if big.len() > x {
            let a = big.pop_first().unwrap();
            sum -= a.sum();
            small.insert(a);
        }
        if idx >= k {
            let left = nums[idx - k];
            let f = *freq.get(&left).unwrap_or(&0);
            let a = Item { f, num: left };
            if !small.remove(&a) {
                big.remove(&a);
                sum -= a.sum();
            }
            let a = Item { f: f - 1, ..a };
            freq.insert(a.num, a.f);
            if a.f > 0 {
                small.insert(a);
            }
            if big.len() < x {
                if let Some(a) = small.pop_last() {
                    big.insert(a);
                    sum += a.sum();
                }
            }
        }
        if idx >= k - 1 {
            res.push(sum);
        }
    }
    res
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    f: i32,
    num: i32,
}

impl Item {
    fn sum(&self) -> i64 {
        i64::from(self.f) * i64::from(self.num)
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
        assert_eq!(find_x_sum(&[1, 1, 2, 2, 3, 4, 2, 3], 6, 2), [6, 10, 12]);
        assert_eq!(find_x_sum(&[3, 8, 7, 8, 7, 5], 2, 2), [11, 15, 15, 15, 12]);
    }

    #[test]
    fn test() {
        assert_eq!(find_x_sum(&[8, 2, 2], 2, 2), [10, 4]);
    }
}
