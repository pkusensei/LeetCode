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

pub fn slide_with_multiset(nums: &[i64], k: usize, size: usize) -> i64 {
    let mut used = MultiSet::default();
    let mut candids = MultiSet::default();
    let mut curr = 0;
    for &num in &nums[..size] {
        curr += num;
        used.insert(num);
    }
    curr = balance(&mut used, &mut candids, k, curr);
    let mut res = curr;
    for (right, &num) in nums.iter().enumerate().skip(size) {
        let del = nums[right - size];
        if used.remove(del) {
            curr -= del;
        } else {
            candids.remove(del);
        }
        if used.back().is_some_and(|&v| v > num) {
            curr += num;
            used.insert(num);
        } else {
            candids.insert(num);
        }
        curr = balance(&mut used, &mut candids, k, curr);
        res = res.min(curr);
    }
    res
}

fn balance(used: &mut MultiSet, candids: &mut MultiSet, k: usize, mut curr: i64) -> i64 {
    while used.len < k {
        let Some(v) = candids.pop_front() else {
            return curr;
        };
        used.insert(v);
        curr += v;
    }
    while used.len > k {
        let v = used.pop_back().unwrap();
        candids.insert(v);
        curr -= v;
    }
    curr
}

#[derive(Default)]
struct MultiSet {
    map: BTreeMap<i64, i32>,
    len: usize,
}

impl MultiSet {
    fn insert(&mut self, num: i64) {
        *self.map.entry(num).or_insert(0) += 1;
        self.len += 1;
    }

    fn remove(&mut self, num: i64) -> bool {
        let Some(v) = self.map.get_mut(&num) else {
            return false;
        };
        *v -= 1;
        if *v == 0 {
            self.map.remove(&num);
        }
        self.len -= 1;
        true
    }

    fn front(&self) -> Option<&i64> {
        self.map.first_key_value().map(|(k, _)| k)
    }

    fn back(&self) -> Option<&i64> {
        self.map.last_key_value().map(|(k, _)| k)
    }

    fn pop_front(&mut self) -> Option<i64> {
        let Some(&v) = self.front() else { return None };
        self.remove(v);
        Some(v)
    }

    fn pop_back(&mut self) -> Option<i64> {
        let Some(&v) = self.back() else { return None };
        self.remove(v);
        Some(v)
    }
}

pub fn minimum_cost(nums: &[i32], k: i32, dist: i32) -> i64 {
    let [k, dist] = [k, dist].map(|v| v as usize);
    let res = i64::from(nums[0]);
    res + slide_with_multiset(
        &nums[1..].iter().map(|&v| i64::from(v)).collect::<Vec<_>>(),
        k - 1,
        1 + dist,
    )
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
        assert_eq!(slide_with_multiset(&[3, 2, 6, 4, 2], 2, 4), 4);
        assert_eq!(slide_with_multiset(&[1, 2, 2, 2, 1], 3, 4), 5);
        assert_eq!(slide_with_multiset(&[8, 18, 9], 2, 2), 26);

        assert_eq!(minimum_cost(&[1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(minimum_cost(&[10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(minimum_cost(&[10, 8, 18, 9], 3, 1), 36);
    }

    #[test]
    fn test() {}
}
