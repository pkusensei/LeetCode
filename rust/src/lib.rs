mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_majority_subarrays(nums: &[i32], target: i32) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut ft = FenwickTree::new(n);
    for (right, &num) in nums.iter().enumerate() {
        if num == target {
            ft.update(right, 1);
        }
        for left in 0..=right {
            let f = ft.query(right) - if left > 0 { ft.query(left - 1) } else { 0 };
            res += i32::from((2 * f) as usize > right + 1 - left)
        }
    }
    res
}

struct FenwickTree {
    tree: Vec<i32>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
            n,
        }
    }

    fn update(&mut self, mut idx: usize, val: i32) {
        idx += 1;
        while idx <= self.n {
            self.tree[idx] += val;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i32 {
        idx += 1;
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
    fn test() {}
}
