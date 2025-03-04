mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_operations(nums: &[i32]) -> i32 {
    let [mut evens, mut odds] = [0, 1].map(|_| HashMap::new());
    let [mut even_max_count1, mut even_max, mut even_max_count2] = [0; 3];
    let [mut odd_max_count1, mut odd_max, mut odd_max_count2] = [0; 3];
    for (i, &num) in nums.iter().enumerate() {
        if i & 1 == 0 {
            let v = evens.entry(num).or_insert(0);
            *v += 1;
            if *v > even_max_count1 {
                even_max_count1 = *v;
                even_max = num;
            }
        } else {
            let v = odds.entry(num).or_insert(0);
            *v += 1;
            if *v > odd_max_count1 {
                odd_max_count1 = *v;
                odd_max = num;
            }
        }
    }
    let n = nums.len() as i32;
    if even_max != odd_max {
        return n - even_max_count1 - odd_max_count1;
    }
    for (&k, &count) in evens.iter() {
        if k != even_max {
            even_max_count2 = even_max_count2.max(count);
        }
    }
    for (&k, &count) in odds.iter() {
        if k != odd_max {
            odd_max_count2 = odd_max_count2.max(count);
        }
    }
    let max_count = (even_max_count1 + odd_max_count2).max(even_max_count2 + odd_max_count1);
    n - max_count
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
        assert_eq!(minimum_operations(&[3, 1, 3, 2, 4, 3]), 3);
        assert_eq!(minimum_operations(&[1, 2, 2, 2, 2]), 2);
    }

    #[test]
    fn test() {
        assert_eq!(minimum_operations(&[2, 2, 2, 2]), 2);
    }
}
