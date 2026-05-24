mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_pairs(nums1: &[i32], nums2: &[i32], queries: &[&[i32]]) -> Vec<i32> {
    let mut nums2: Vec<_> = nums2.iter().map(|&v| i64::from(v)).collect();
    let n = nums2.len();
    // +1 so that when `n==2` does not cause index out of range
    let b = 1 + n.isqrt();
    let mut blocks = vec![HashMap::new(); 1 + b];
    for (i, &num) in nums2.iter().enumerate() {
        *blocks[i / b].entry(num).or_insert(0) += 1;
    }
    let mut lazy = vec![0; 1 + b];
    let mut res = vec![];
    for q in queries {
        if q[0] == 1 {
            let [left, right] = [1, 2].map(|i| q[i] as usize);
            let val = i64::from(q[3]);
            update(&mut nums2, &mut blocks, &mut lazy, left, right, val);
        } else {
            let mut curr = 0;
            for &num in nums1.iter() {
                let target = i64::from(q[1] - num);
                curr += query(&blocks, &lazy, target);
            }
            res.push(curr);
        }
    }
    res
}

fn update(
    nums2: &mut [i64],
    blocks: &mut [HashMap<i64, i32>],
    lazy: &mut [i64],
    mut left: usize,
    right: usize,
    val: i64,
) {
    let b = 1 + nums2.len().isqrt();
    while left <= right {
        if left % b == 0 && left + b - 1 <= right {
            lazy[left / b] += i64::from(val);
            left += b;
        } else {
            let old = nums2[left];
            let v = blocks[left / b].entry(old).or_insert(0);
            *v -= 1;
            if *v == 0 {
                blocks[left / b].remove(&old);
            }
            *blocks[left / b].entry(old + val).or_insert(0) += 1;
            nums2[left] += val;
            left += 1;
        }
    }
}

fn query(blocks: &[HashMap<i64, i32>], lazy: &[i64], target: i64) -> i32 {
    let mut res = 0;
    for (map, tag) in blocks.iter().zip(lazy) {
        res += map.get(&(target - tag)).unwrap_or(&0)
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
        assert_eq!(
            number_of_pairs(&[1, 2], &[3, 4], &[&[2, 5], &[1, 0, 0, 2], &[2, 5]]),
            [2, 1]
        )
    }

    #[test]
    fn test() {
        assert_eq!(
            number_of_pairs(
                &[49, 10],
                &[
                    1, 10, 30, 9, 32, 7, 42, 1, 36, 12, 11, 32, 51, 47, 29, 12, 23, 50, 41, 5, 1,
                    18, 50, 23, 10, 32, 53
                ],
                &[
                    &[1, 11, 24, 30],
                    &[2, 35],
                    &[1, 1, 21, 31],
                    &[2, 93],
                    &[2, 102]
                ]
            ),
            [0, 0, 2]
        )
    }
}
