mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_pairs(nums: &[i32]) -> i32 {
    use itertools::Itertools;
    use std::collections::BTreeMap;
    let map: BTreeMap<i64, usize> = nums
        .iter()
        .copied()
        .sorted_unstable_by(|a, b| b.cmp(a)) // Big number gets smaller idx
        .dedup()
        .enumerate()
        .map(|(i, v)| (i64::from(v), 1 + i)) // +1 for Fenwick Tree
        .collect();
    dbg!(&map);
    let mut ft = FenwickTree::new(map.len());
    let mut res = 0;
    for &num in nums.iter() {
        if let Some((_, &i)) = map.range(1 + 2 * i64::from(num)..).next() {
            res += ft.query(i);
        };
        ft.update(map[&i64::from(num)], 1);
    }
    res
}

struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
        }
    }

    fn update(&mut self, mut idx: usize, val: i32) {
        let sz = self.tree.len();
        while idx < sz {
            self.tree[idx] += val;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i32 {
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
        assert_eq!(reverse_pairs(&[1, 3, 2, 3, 1]), 2);
    }

    #[test]
    fn test() {}
}
