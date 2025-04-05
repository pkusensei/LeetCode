mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::{cmp::Reverse, collections::BTreeMap};

pub fn maximum_sum_queries(nums1: &[i32], nums2: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let nums = nums1
        .iter()
        .zip(nums2.iter())
        .map(|(&x, &y)| [x, y])
        .sorted_unstable_by_key(|v| Reverse(v[0]))
        .collect_vec();
    let n = queries.len();
    let qids = (0..n)
        .sorted_unstable_by_key(|&i| Reverse(queries[i][0]))
        .collect_vec();
    let mut res = vec![-1; n];
    let mut map = BTreeMap::new(); // (y, x+y)
    let mut ni = 0;
    for qi in qids {
        let [qx, qy] = queries[qi][..] else {
            unreachable!()
        };
        while nums.get(ni).is_some_and(|&v| v[0] >= qx) {
            let [x, y] = nums[ni];
            // try all [x,y] with x>=qx
            update(&mut map, y, x + y);
            ni += 1;
        }
        res[qi] = map.range(qy..).next().map(|p| *p.1).unwrap_or(-1);
    }
    res
}

fn update(map: &mut BTreeMap<i32, i32>, y: i32, sum: i32) {
    if map.range(y..).next().is_some() {
        // With x sorted descending
        // this y being not the largest means
        // there exists [x1,y1] with x1+y1>=sum
        return;
    }
    while let Some((&k, &v)) = map.range(..=y).next_back() {
        // Remove all previous [x1,y1] with y1<=y && x1+y1<=sum
        if v <= sum {
            map.remove(&k);
        } else {
            break;
        }
    }
    map.insert(y, sum);
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
            maximum_sum_queries(&[4, 3, 1, 2], &[2, 4, 9, 5], &[[4, 1], [1, 3], [2, 5]]),
            [6, 10, 7]
        );
        assert_eq!(
            maximum_sum_queries(&[3, 2, 5], &[2, 3, 4], &[[4, 4], [3, 2], [1, 1]],),
            [9, 9, 9]
        );
        assert_eq!(maximum_sum_queries(&[2, 1], &[2, 3], &[[3, 3]]), [-1]);
    }

    #[test]
    fn test() {}
}
