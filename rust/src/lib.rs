mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(nums: Vec<i32>, s: String) -> i64 {
    use itertools::izip;
    use std::collections::BinaryHeap;
    let mut res = 0;
    let mut heap = BinaryHeap::new();
    for (&num, b) in izip!(&nums, s.bytes()) {
        heap.push(num);
        if b == b'1' {
            res += i64::from(heap.pop().unwrap_or_default());
        }
    }
    res
}

// q2
pub fn maximum_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by(|a, b| b.cmp(a));
    let mut res = 0;
    let mut rems = [None::<i32>; 3];
    for (i, &a) in nums.iter().enumerate() {
        let rem = (3 - a % 3) % 3;
        if let Some(v) = rems[rem as usize] {
            res = res.max(v + a);
        }
        if rems.iter().any(|v| v.is_none()) {
            for b in &nums[..i] {
                let rem = (a + b) % 3;
                let v = rems[rem as usize].get_or_insert(a + b);
                *v = (*v).max(a + b);
            }
        }
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
        assert_eq!(maximum_sum(vec![4, 2, 3, 1]), 9);
        assert_eq!(maximum_sum(vec![2, 1, 5]), 0);
    }

    #[test]
    fn test() {}
}
