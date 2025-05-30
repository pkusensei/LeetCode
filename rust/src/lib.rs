mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn subsequences_with_middle_mode(nums: &[i32]) -> i32 {
    let n = nums.len();
    let f = |mut acc: HashMap<i32, i64>, &v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    };
    let mut left = nums[..2].iter().fold(HashMap::new(), f);
    let mut right = nums[2..].iter().fold(HashMap::new(), f);
    let mut res = 0;
    for idx in 2..n - 2 {
        let num = nums[idx];
        let v = right.entry(num).or_insert(0);
        *v -= 1;
        if *v == 0 {
            right.remove(&num);
        }
        let [left_count, right_count] = [&left, &right].map(|m| *m.get(&num).unwrap_or(&0));
        let left_other = idx as i64 - left_count;
        let right_other = (n - idx) as i64 - 1 - right_count;

        // mode 5
        res = (res + n_choose_2(left_count) * n_choose_2(right_count)) % M;

        // mode 4
        // [a a] a [a ?]
        res = (res + n_choose_2(left_count) * right_count % M * right_other % M) % M;
        // [a ?] a [a a]
        res = (res + left_count * left_other % M * n_choose_2(right_count) % M) % M;

        // mode 3
        // [a a] a [? ?]
        res = (res + n_choose_2(left_count) * n_choose_2(right_other) % M) % M;
        // [? ?] a [a a]
        res = (res + n_choose_2(left_other) * n_choose_2(right_count) % M) % M;
        // [a ?] a [a ?]
        res = (res + left_count * left_other % M * right_count % M * right_other % M) % M;

        // mode 2
        // [a ?] a [? ?]
        res = (res + left_count * find_2(num, left_other, right_other, &left, &right) % M) % M;
        // [? ?] a [a ?]
        res = (res + right_count * find_2(num, right_other, left_other, &right, &left) % M) % M;

        *left.entry(num).or_insert(0) += 1;
    }
    res as i32
}

const M: i64 = 1_000_000_007;

const fn n_choose_2(n: i64) -> i64 {
    n * (n - 1) / 2 % M
}

fn find_2(
    mode: i32,
    other1: i64,
    other2: i64,
    freqs1: &HashMap<i32, i64>,
    freqs2: &HashMap<i32, i64>,
) -> i64 {
    // [a ?] a [? ?]
    let mut res = other1 * n_choose_2(other2) % M;
    for (&num, &count1) in freqs1.iter().filter(|&(&num, _)| num != mode) {
        let count2 = *freqs2.get(&num).unwrap_or(&0);
        // Exclude [a x] a [x x]
        res = (res - count1 * n_choose_2(count2) % M).rem_euclid(M);
        // Exclude [a x] a [x ?]
        res = (res - count1 * count2 % M * (other2 - count2) % M).rem_euclid(M);
    }
    for (&num, &count2) in freqs2.iter().filter(|&(&num, _)| num != mode) {
        let count1 = *freqs1.get(&num).unwrap_or(&0);
        // Exclude [a ?] a [x x]
        res = (res - (other1 - count1) * n_choose_2(count2) % M).rem_euclid(M);
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
        assert_eq!(subsequences_with_middle_mode(&[1, 1, 1, 1, 1, 1]), 6);
        assert_eq!(subsequences_with_middle_mode(&[1, 2, 2, 3, 3, 4]), 4);
        assert_eq!(
            subsequences_with_middle_mode(&[0, 1, 2, 3, 4, 5, 6, 7, 8]),
            0
        );
    }

    #[test]
    fn test() {}
}
