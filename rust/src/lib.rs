mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut right = nums.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let mut left = HashMap::new();
    let mut res = vec![];
    for &num in nums.iter() {
        *left.entry(num).or_insert(0) += 1;
        right.entry(num).and_modify(|v| *v -= 1);
        if right[&num] == 0 {
            right.remove(&num);
        }
        res.push(left.len() as i32 - right.len() as i32);
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
