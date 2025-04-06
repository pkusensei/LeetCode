mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_imbalance_numbers(nums: &[i32]) -> i32 {
    use std::collections::BTreeSet;
    let mut res = 0;
    for (i1, &a) in nums.iter().enumerate() {
        let mut map = BTreeSet::from([a]);
        let mut curr = 0;
        for &b in nums.iter().skip(1 + i1) {
            match (map.range(..=b).next_back(), map.range(b..).next()) {
                (Some(&left), None) => curr += i32::from(b - left > 1),
                (None, Some(&right)) => curr += i32::from(right - b > 1),
                (Some(&left), Some(&right)) if b == left || b == right => (),
                (Some(&left), Some(&right)) => match (b - left > 1, right - b > 1) {
                    (true, true) => curr += 1,
                    (true, false) | (false, true) => (),
                    (false, false) => curr -= 1,
                },
                _ => (),
            }
            res += curr;
            map.insert(b);
        }
    }
    res
}

pub fn with_vec(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    for (i1, &a) in nums.iter().enumerate() {
        let mut seen = vec![false; 2 + n];
        let mut curr = 0;
        seen[a as usize] = true;
        for &b in nums.iter().skip(1 + i1) {
            let b = b as usize;
            if !seen[b] {
                seen[b] = true;
                if seen[b - 1] && seen[b + 1] {
                    curr -= 1;
                } else if !seen[b - 1] && !seen[b + 1] {
                    curr += 1;
                }
            }
            res += curr;
        }
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
        assert_eq!(sum_imbalance_numbers(&[2, 3, 1, 4]), 3);
        assert_eq!(sum_imbalance_numbers(&[1, 3, 3, 3, 5]), 8);

        assert_eq!(with_vec(&[2, 3, 1, 4]), 3);
        assert_eq!(with_vec(&[1, 3, 3, 3, 5]), 8);
    }

    #[test]
    fn test() {
        assert_eq!(sum_imbalance_numbers(&[1, 3, 2]), 1);
        assert_eq!(with_vec(&[1, 3, 2]), 1);
    }
}
