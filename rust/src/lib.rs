mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn alternating_xor(nums: &[i32], target1: i32, target2: i32) -> i32 {
    use std::collections::HashMap;
    const M: i32 = 1_000_000_007;
    let mut pref_xor = 0;
    let mut map1 = HashMap::new(); // ends in target1
    let mut map2 = HashMap::new(); // ends in target2
    let [mut dp1, mut dp2] = [0, 0];
    for num in nums {
        pref_xor ^= num;
        let temp1 = pref_xor ^ target1;
        let temp2 = pref_xor ^ target2;
        let mut curr1 = *map2.get(&temp1).unwrap_or(&0);
        let curr2 = *map1.get(&temp2).unwrap_or(&0);
        if pref_xor == target1 {
            curr1 += 1;
        }
        let v = map1.entry(pref_xor).or_insert(0);
        *v = (*v + curr1) % M;
        let v = map2.entry(pref_xor).or_insert(0);
        *v = (*v + curr2) % M;
        [dp1, dp2] = [curr1, curr2]
    }
    (dp1 + dp2) % M
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
        assert_eq!(alternating_xor(&[2, 3, 1, 4], 1, 5), 1);
        assert_eq!(alternating_xor(&[1, 0, 0], 1, 0), 3);
        assert_eq!(alternating_xor(&[7], 1, 7), 0);
    }

    #[test]
    fn test() {
        assert_eq!(alternating_xor(&[22867, 87890, 22867], 22867, 87890), 1);
        assert_eq!(alternating_xor(&[12], 12, 1), 1);
        assert_eq!(alternating_xor(&[33646, 79267, 0], 33646, 79267), 1);
    }
}
