mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_majority_subarrays(nums: &[i32], target: i32) -> i64 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let prefix = nums.iter().fold(vec![0], |mut acc, &v| {
        acc.push(if v == target { 1_i32 } else { -1 } + acc.last().unwrap_or(&0));
        acc
    });
    if prefix
        .last()
        .is_some_and(|&v| v < 0 && v.abs() == nums.len() as i32)
    {
        return 0;
    }
    let map: HashMap<_, _> = prefix
        .iter()
        .copied()
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    let n = map.len();
    let mut ft = FenwickTree::new(n);
    let mut res = 0;
    for p in &prefix {
        let i = map[p];
        ft.update(1 + i, 1);
        res += ft.query(i); // find all freq<current
    }
    res
}

struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
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
        assert_eq!(count_majority_subarrays(&[1, 2, 2, 3], 2), 5);
        assert_eq!(count_majority_subarrays(&[1, 1, 1, 1], 1), 10);
    }

    #[test]
    fn test() {
        assert_eq!(count_majority_subarrays(&[7, 1], 7), 1);
    }
}
