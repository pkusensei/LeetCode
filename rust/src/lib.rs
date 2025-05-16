mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_final_state(nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    const M: i64 = 1_000_000_007;
    if multiplier == 1 {
        return nums;
    }
    let n = nums.len();
    let mult = i64::from(multiplier);
    let mut heap = BinaryHeap::with_capacity(n);
    let mut max = i64::MIN;
    for (i, &v) in nums.iter().enumerate() {
        let v = i64::from(v);
        max = max.max(v);
        heap.push(Reverse((v, i)));
    }
    while k > 0 && heap.peek().is_some_and(|&Reverse((v, _))| v * mult <= max) {
        let Reverse((v, i)) = heap.pop().unwrap();
        heap.push(Reverse((v * mult % M, i)));
        k -= 1;
    }
    let mult2 = mod_pow(mult, (k as usize / n) as i64, M);
    let mut res = vec![0; n];
    while let Some(Reverse((mut v, i))) = heap.pop() {
        v = v * mult2 % M;
        if usize::try_from(k).is_ok_and(|_k| _k % n > 0) {
            v = v * mult % M;
            k -= 1;
        }
        res[i] = v as i32;
    }
    res
}

const fn mod_pow(b: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % m, exp >> 1, m)
    } else {
        mod_pow(b * b % m, exp >> 1, m) * b % m
    }
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
        assert_eq!(get_final_state(vec![2, 1, 3, 5, 6], 5, 2), [8, 4, 6, 5, 6]);
        assert_eq!(
            get_final_state(vec![100000, 2000], 2, 1000000),
            [999999307, 999999993]
        );
    }

    #[test]
    fn test() {}
}
