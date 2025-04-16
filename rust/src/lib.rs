mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_balanced_subsequence_sum(nums: &[i64]) -> i64 {
    use std::collections::BTreeMap;
    let arr: Vec<_> = nums
        .iter()
        .enumerate()
        .map(|(i, &num)| i64::from(num) - i as i64)
        .collect();
    // nums[i]-i => max_sum up to i
    let mut map = BTreeMap::new();
    let mut res = i64::MIN;
    for (&num, &val) in nums.iter().zip(arr.iter()) {
        let mut curr_sum = i64::from(num);
        if curr_sum <= 0 {
            res = res.max(curr_sum); // negative => record only single number
            continue;
        }
        if let Some((_, &prev)) = map.range(..=val).next_back() {
            curr_sum += prev; // max prev_sum up until i
        }
        let del: Vec<_> = map
            .range(val..)
            .take_while(|(_, v)| **v < curr_sum)
            .map(|(&k, _)| k)
            .collect(); // monotonic map
        for k in del {
            map.remove(&k);
        }
        map.insert(val, curr_sum);
        res = res.max(curr_sum);
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
        assert_eq!(max_balanced_subsequence_sum(&[3, 3, 5, 6]), 14);
        assert_eq!(max_balanced_subsequence_sum(&[5, -1, -3, 8]), 13);
        assert_eq!(max_balanced_subsequence_sum(&[-2, -1]), -1);
    }

    #[test]
    fn test() {}
}
