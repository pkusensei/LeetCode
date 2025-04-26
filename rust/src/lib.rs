mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unmarked_sum_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut sum: i64 = nums.iter().map(|&v| i64::from(v)).sum();
    let mut seen = vec![false; nums.len()];
    let mut heap: BinaryHeap<_> = nums
        .iter()
        .enumerate()
        .map(|(i, &v)| Reverse((v, i)))
        .collect();
    let mut res = vec![];
    for q in queries {
        let [i, mut k] = q[..] else {
            unreachable!();
        };
        if !seen[i as usize] {
            sum -= i64::from(nums[i as usize]);
            seen[i as usize] = true
        }
        if k > 0 {
            while let Some(Reverse((v, i))) = heap.pop() {
                if seen[i] {
                    continue;
                }
                seen[i] = true;
                sum -= i64::from(v);
                k -= 1;
                if k <= 0 {
                    break;
                }
            }
        }
        res.push(sum);
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
    fn basics() {}

    #[test]
    fn test() {}
}
