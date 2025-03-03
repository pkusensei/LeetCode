mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_difference(nums: &[i32]) -> i64 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = nums.len() / 3;
    let mut pre_min = Vec::with_capacity(n);
    let mut curr: i64 = nums[..n].iter().map(|&v| i64::from(v)).sum();
    pre_min.push(curr);
    let mut max_heap: BinaryHeap<_> = nums[..n].iter().copied().collect();
    for idx in 1..=n {
        let val = nums[idx + n - 1];
        curr += i64::from(val);
        max_heap.push(val);
        let top = max_heap.pop().unwrap();
        curr -= i64::from(top);
        pre_min.push(curr);
    }

    let mut suf_max = Vec::with_capacity(n);
    curr = nums[2 * n..].iter().map(|&v| i64::from(v)).sum();
    suf_max.push(curr);
    let mut min_heap: BinaryHeap<_> = nums[2 * n..].iter().map(|&v| Reverse(v)).collect();
    for idx in (n..2 * n).rev() {
        let val = nums[idx];
        curr += i64::from(val);
        min_heap.push(Reverse(val));
        let Reverse(top) = min_heap.pop().unwrap();
        curr -= i64::from(top);
        suf_max.push(curr);
    }
    suf_max.reverse();
    pre_min
        .into_iter()
        .zip(suf_max)
        .map(|(a, b)| a - b)
        .min()
        .unwrap()
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
        assert_eq!(minimum_difference(&[3, 1, 2]), -1);
        assert_eq!(minimum_difference(&[7, 9, 5, 8, 1, 3]), 1);
    }

    #[test]
    fn test() {}
}
