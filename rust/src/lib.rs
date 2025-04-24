mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_subarray_sum(nums: &[i32], k: i32) -> i64 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i64> = HashMap::new();
    let mut prefix = 0;
    let mut res = i64::MIN;
    for &num in nums.iter() {
        if let Some(v) = map.get_mut(&num) {
            *v = (*v).min(prefix);
        } else {
            map.insert(num, prefix);
        }
        prefix += i64::from(num);
        if let Some(&v) = map.get(&(num - k)) {
            res = res.max(prefix - v);
        }
        if let Some(&v) = map.get(&(num + k)) {
            res = res.max(prefix - v);
        }
    }
    if res == i64::MIN { 0 } else { res }
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
        assert_eq!(maximum_subarray_sum(&[1, 2, 3, 4, 5, 6], 1), 11);
        assert_eq!(maximum_subarray_sum(&[-1, 3, 2, 4, 5], 3), 11);
        assert_eq!(maximum_subarray_sum(&[-1, -2, -3, -4], 2), -6);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_subarray_sum(&[1, 5], 2), 0);
    }
}
