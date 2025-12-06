mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_inversion_count(nums: &[i32], k: i32) -> i64 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let num_idx: HashMap<_, _> = nums
        .iter()
        .copied()
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, v)| (v, 1 + i))
        .collect();
    let mut res = i64::MAX;
    let n = num_idx.len();
    let k = k as usize;
    let mut ft = FenwickTree::new(n);
    let mut curr = 0;
    for (right, num) in nums.iter().enumerate() {
        if right >= k {
            let left = num_idx[&nums[right - k]];
            ft.update(left, -1);
            curr -= ft.query(left - 1);
        }
        curr += right.min(k - 1) as i64 - ft.query(num_idx[num]);
        ft.update(num_idx[num], 1);
        if right >= k - 1 {
            res = res.min(curr);
        }
    }
    res
}

struct FenwickTree {
    tree: Vec<i64>,
    n: usize, // Semantic size
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
            n,
        }
    }

    fn update(&mut self, mut idx: usize, val: i64) {
        while idx <= self.n {
            self.tree[idx] += val;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut res = 0;
        while idx > 0 {
            res += self.tree[idx];
            idx -= idx & idx.wrapping_neg();
        }
        res
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
        assert_eq!(min_inversion_count(&[3, 1, 2, 5, 4], 3), 0);
        assert_eq!(min_inversion_count(&[5, 3, 2, 1], 4), 6);
    }

    #[test]
    fn test() {}
}
