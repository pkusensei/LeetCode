mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_partitions_after_operations(s: &str, k: i32) -> i32 {
    let n = s.len();
    let mut left = Vec::with_capacity(n);
    let [mut partitions, mut mask, mut pop_count] = [0; 3];
    left.push([partitions, mask, pop_count]);
    for b in s.bytes().take(n - 1) {
        let bit = 1 << (b - b'a');
        if mask & bit == 0 {
            pop_count += 1;
            if pop_count <= k {
                mask |= bit;
            } else {
                partitions += 1;
                mask = bit;
                pop_count = 1;
            }
        }
        left.push([partitions, mask, pop_count]);
    }
    let mut right = Vec::with_capacity(n);
    [partitions, mask, pop_count] = [0; 3];
    right.push([partitions, mask, pop_count]);
    for b in s.bytes().rev().take(n - 1) {
        let bit = 1 << (b - b'a');
        if mask & bit == 0 {
            pop_count += 1;
            if pop_count <= k {
                mask |= bit;
            } else {
                partitions += 1;
                mask = bit;
                pop_count = 1;
            }
        }
        right.push([partitions, mask, pop_count]);
    }
    right.reverse();
    let mut res = 0;
    for (left, right) in left.into_iter().zip(right) {
        // baseline 2 + left_acc [..i] + right_acc [1+i..]
        // one partial forward segment and one partial backward segment
        let mut seg = 2 + left[0] + right[0];
        let mask = left[1] | right[1];
        if left[2] == k && right[2] == k && mask.count_ones() < 26 {
            // left and right side of this segment is already k distinct
            // change current [i] to force partition
            seg += 1;
        } else if (1 + mask.count_ones()).min(26) as i32 <= k {
            seg -= 1; // continue/stretch left segment onto right
        }
        res = res.max(seg);
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
        assert_eq!(max_partitions_after_operations("accca", 2), 3);
        assert_eq!(max_partitions_after_operations("aabaab", 3), 1);
        assert_eq!(max_partitions_after_operations("xxyz", 1), 4);
    }

    #[test]
    fn test() {}
}
