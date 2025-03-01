mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn recover_array(nums: &mut [i32]) -> Vec<i32> {
    let count = nums.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    nums.sort_unstable();
    for num in nums.iter().skip(1) {
        let d = num - nums[0];
        if d > 0 && d & 1 == 0 {
            if let Some(v) = check(nums, d / 2, count.clone()) {
                return v;
            }
        }
    }
    vec![]
}

fn check(nums: &[i32], k: i32, mut count: HashMap<i32, i32>) -> Option<Vec<i32>> {
    let mut res = vec![];
    for num in nums {
        if count.get(num).is_some_and(|&v| v == 0) {
            continue;
        }
        if !count.get(&(num + 2 * k)).is_some_and(|&v| v > 0) {
            return None;
        }
        count.entry(*num).and_modify(|v| *v -= 1);
        count.entry(num + 2 * k).and_modify(|v| *v -= 1);
        res.push(num + k);
    }
    Some(res)
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
        assert_eq!(recover_array(&mut [2, 10, 6, 4, 8, 12]), [3, 7, 11]);
        assert_eq!(recover_array(&mut [1, 1, 3, 3]), [2, 2]);
        assert_eq!(recover_array(&mut [5, 435]), [220]);
    }

    #[test]
    fn test() {}
}
