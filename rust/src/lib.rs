mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarray_sum(nums: &[i32]) -> i64 {
    let mut map = HashMap::<_, Vec<usize>>::new();
    let mut sum = 0;
    let mut max = i64::MIN;
    for (idx, &num) in nums.iter().enumerate() {
        let num = i64::from(num);
        sum += num;
        max = max.max(num);
        if num < 0 {
            map.entry(num).or_default().push(idx);
        }
    }
    if max < 0 {
        return max;
    }
    if map.is_empty() {
        return sum;
    }
    let mut tree = SegmentTree::new(nums);
    let mut res = i64::MIN;
    for (num, idx) in map {
        for &i in &idx {
            tree.update(i, 0);
        }
        res = res.max(tree.tree[1].max_sum);
        for i in idx {
            tree.update(i, num);
        }
    }
    res
}

#[derive(Default, Clone, Copy)]
struct Node {
    subarr_sum: i64,
    max_sum: i64,
    max_pref: i64,
    max_suf: i64,
}

impl Node {
    fn new(val: i64) -> Self {
        Self {
            subarr_sum: val,
            max_sum: val,
            max_pref: val,
            max_suf: val,
        }
    }
}

struct SegmentTree {
    tree: Vec<Node>,
    n: usize,
}

impl SegmentTree {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut tree = Self {
            tree: vec![Node::default(); 4 * n],
            n,
        };
        for (idx, &num) in nums.iter().enumerate() {
            tree.update(idx, num.into());
        }
        tree
    }

    fn update(&mut self, idx: usize, val: i64) {
        self.update_impl(1, 0, self.n - 1, idx, val);
    }

    fn update_impl(&mut self, pos: usize, left: usize, right: usize, idx: usize, val: i64) {
        if left == right {
            self.tree[pos] = Node::new(val);
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self.update_impl(2 * pos, left, mid, idx, val);
        } else {
            self.update_impl(2 * pos + 1, 1 + mid, right, idx, val);
        }
        let node1 = self.tree[2 * pos];
        let node2 = self.tree[2 * pos + 1];
        let subarr_sum = node1.subarr_sum + node2.subarr_sum;
        let max_sum = node1
            .max_sum
            .max(node2.max_sum)
            .max(node1.max_suf + node2.max_pref);
        let max_pref = node1.max_pref.max(node1.subarr_sum + node2.max_pref);
        let max_suf = node2.max_suf.max(node2.subarr_sum + node1.max_suf);
        self.tree[pos] = Node {
            subarr_sum,
            max_sum,
            max_pref,
            max_suf,
        };
    }
}

pub fn kadanes(nums: &[i32]) -> i64 {
    let mut res = i64::MIN;
    // minimum possible prefix linked to a number that could be removed
    // i.e remove num, and delete this prefix to get max result
    // zero => no removal
    let mut map = HashMap::from([(0_i64, 0_i64)]);
    let mut prefix = 0;
    let mut lowest = 0;
    for &num in nums {
        let num = i64::from(num);
        prefix += num;
        res = res.max(prefix - lowest); // Kadane's
        if num < 0 {
            // When encountering a negative number
            let curr = *map.get(&num).unwrap_or(&0);
            let zero = *map.get(&0).unwrap_or(&0);
            // Try updating a new min prefix that could be deleted
            // if this number were to be removed
            let val = num + curr.min(zero);
            let v = map.entry(num).or_insert(0);
            *v = (*v).min(val);
            lowest = lowest.min(val);
        }
        // Always update no-removal case
        let zero = map.entry(0).or_insert(0);
        *zero = (*zero).min(prefix);
        lowest = lowest.min(*zero);
    }
    res
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
        assert_eq!(max_subarray_sum(&[-3, 2, -2, -1, 3, -2, 3]), 7);
        assert_eq!(max_subarray_sum(&[1, 2, 3, 4]), 10);

        assert_eq!(kadanes(&[-3, 2, -2, -1, 3, -2, 3]), 7);
        assert_eq!(kadanes(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn test() {
        assert_eq!(max_subarray_sum(&[-2, -2, -2]), -2);
        assert_eq!(kadanes(&[-2, -2, -2]), -2);
    }
}
