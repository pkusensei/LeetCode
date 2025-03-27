mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
    use std::{collections::BTreeMap, iter};
    let mut map = BTreeMap::<i32, i32>::new();
    for &num in basket1.iter() {
        *map.entry(num).or_insert(0) += 1;
    }
    for &num in basket2.iter() {
        *map.entry(num).or_insert(0) -= 1;
    }
    let mut nums = vec![]; // nums to be swapped
    for (k, v) in map.iter().map(|(&k, &v)| (k, v.unsigned_abs() as usize)) {
        if v & 1 == 1 {
            return -1;
        }
        nums.extend(iter::repeat_n(k, v / 2));
    }
    let mut res = 0;
    // min of both arrays
    let min = *map.keys().next().unwrap_or(&10i32.pow(9));
    // Of all nums to be swapped, try either
    // 1) smaller num paired with bigger num => take(n/2)
    //    [4,4] and [3,3] => 2*[3,4] with min cost of 3
    // 2) pair big num with min element, and swap twice => 2*min
    //    [1, 3,3] and [1, 4,4]
    //    1st swap => [4, 3, 3] and [1, 1, 4]
    //    2nd swap => [4, 1, 3] and [1, 3, 4]
    for &num in nums.iter().take(nums.len() / 2) {
        res += i64::from(num.min(2 * min));
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
        assert_eq!(min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]), 1);
        assert_eq!(min_cost(vec![2, 3, 4, 1], vec![3, 2, 5, 1]), -1);
    }

    #[test]
    fn test() {}
}
