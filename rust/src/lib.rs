mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_sum_square_diff(nums1: &[i32], nums2: &[i32], k1: i32, k2: i32) -> i64 {
    use std::collections::{BinaryHeap, HashMap};
    let mut heap: BinaryHeap<_> = nums1
        .iter()
        .zip(nums2.iter())
        .fold(HashMap::new(), |mut acc, (a, b)| {
            *acc.entry(i64::from(a.abs_diff(*b))).or_insert(0_i64) += 1;
            acc
        })
        .into_iter()
        .filter_map(|(num, count)| if num > 0 { Some((num, count)) } else { None })
        .collect();
    let mut k = i64::from(k1 + k2);
    while let Some((num, mut count)) = heap.pop() {
        let d = count.min(k);
        count -= d;
        if count > 0 {
            heap.push((num, count));
        }
        if heap.peek().is_some_and(|&(n, _c)| n == num - 1) {
            let (n, c) = heap.pop().unwrap();
            heap.push((n, c + d));
        } else if num > 1 {
            heap.push((num - 1, d));
        }
        k -= d;
        if k == 0 {
            break;
        }
    }
    heap.into_iter()
        .map(|(num, count)| num.pow(2) * count)
        .sum()
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
        assert_eq!(
            min_sum_square_diff(&[1, 4, 10, 12], &[5, 8, 6, 9], 1, 1),
            43
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            min_sum_square_diff(&[10, 10, 10, 11, 5], &[1, 0, 6, 6, 1], 11, 27),
            0
        );
    }
}
