mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn result_array(nums: &[i32]) -> Vec<i32> {
    use itertools::Itertools;
    use std::collections::HashMap;
    let sorted = nums.iter().copied().sorted().dedup().collect_vec();
    // 1-based indexing
    let map: HashMap<_, _> = sorted
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, 1 + i))
        .collect();
    let len = sorted.len();
    let mut arr1 = Arr::new(len, map[&nums[0]]);
    let mut arr2 = Arr::new(len, map[&nums[1]]);
    for &num in &nums[2..] {
        let val = map[&num];
        match arr1.count_greater(val).cmp(&arr2.count_greater(val)) {
            std::cmp::Ordering::Less => arr2.push(val),
            std::cmp::Ordering::Equal => {
                if arr1.nums.len() <= arr2.nums.len() {
                    arr1.push(val);
                } else {
                    arr2.push(val);
                }
            }
            std::cmp::Ordering::Greater => arr1.push(val),
        }
    }
    // offset 1-based indexing
    arr1.nums
        .into_iter()
        .chain(arr2.nums)
        .map(|i| sorted[i - 1])
        .collect()
}

struct Arr {
    ft: fenwick_tree::FenwickTree,
    nums: Vec<usize>,
}

impl Arr {
    fn new(max_val: usize, init: usize) -> Self {
        let nums = vec![init];
        let mut ft = fenwick_tree::FenwickTree::new(max_val);
        ft.update(init, 1);
        Self { ft, nums }
    }

    fn push(&mut self, val: usize) {
        self.ft.update(val, 1);
        self.nums.push(val);
    }

    fn count_greater(&self, val: usize) -> i32 {
        self.nums.len() as i32 - self.ft.query(val)
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
        assert_eq!(result_array(&[2, 1, 3, 3]), [2, 3, 1, 3]);
        // assert_eq!(result_array(&[5, 14, 3, 1, 2]), [5, 3, 1, 2, 14]);
        // assert_eq!(result_array(&[3, 3, 3, 3]), [3, 3, 3, 3]);
    }

    #[test]
    fn test() {}
}
