mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn ways_to_partition(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut prefix = Vec::with_capacity(n);
    let mut num_diff = Vec::with_capacity(n);
    for (idx, &num) in nums.iter().enumerate() {
        prefix.push(i64::from(num) + prefix.last().unwrap_or(&0));
        if k != num {
            num_diff.push((idx, i64::from(k - num)));
        }
    }
    let mut res = 0;
    let mut zero = 0;
    let mut part_diff = HashMap::<_, Vec<_>>::new();
    for p in 1..n {
        let d = prefix[n - 1] - prefix[p - 1] * 2;
        zero += i32::from(d == 0);
        part_diff.entry(d).or_default().push(p);
    }
    for (idx, nd) in num_diff {
        let mut curr = 0;
        if let Some(v) = part_diff.get(&nd) {
            let i = v.partition_point(|&piv| piv <= idx);
            curr += v.len() - i;
        }
        if let Some(p) = part_diff
            .get(&-nd)
            .map(|v| v.partition_point(|&piv| piv <= idx))
        {
            curr += p;
        }
        res = res.max(curr as i32);
    }
    res.max(zero)
}

pub fn linear_time(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let [mut left, mut right] = [0, 1].map(|_| HashMap::new());
    let sum = nums.iter().fold(0, |acc, &num| acc + i64::from(num));
    let mut left_sum = 0;
    for &num in nums[..n - 1].iter() {
        left_sum += i64::from(num);
        *right.entry(sum - 2 * left_sum).or_insert(0) += 1;
    }
    let mut res = *right.get(&0).unwrap_or(&0);
    left_sum = 0;
    for &num in nums.iter() {
        res = res.max(
            left.get(&i64::from(num - k)).unwrap_or(&0)
                + right.get(&i64::from(k - num)).unwrap_or(&0),
        );
        left_sum += i64::from(num);
        *left.entry(sum - 2 * left_sum).or_insert(0) += 1;
        *right.entry(sum - 2 * left_sum).or_insert(0) -= 1;
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
    fn basics() {
        assert_eq!(ways_to_partition(&[2, -1, 2], 3), 1);
        assert_eq!(ways_to_partition(&[0, 0, 0], 1), 2);
        assert_eq!(
            ways_to_partition(
                &[22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14],
                -33
            ),
            4
        );

        assert_eq!(linear_time(&[2, -1, 2], 3), 1);
        assert_eq!(linear_time(&[0, 0, 0], 1), 2);
        assert_eq!(
            linear_time(
                &[22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14],
                -33
            ),
            4
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            ways_to_partition(
                &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30827, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
                0
            ),
            33
        );

        assert_eq!(
            linear_time(
                &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30827, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
                0
            ),
            33
        );
    }
}
