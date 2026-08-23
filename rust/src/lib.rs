mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subarray(nums: &[i32], k: i32) -> i32 {
    let mut left = 0;
    let mut res = 0;
    let mut seen = vec![];
    let mut freq = vec![0; 1 + M];
    let mut distinct = 0;
    for (right, &num) in nums.iter().enumerate() {
        let curr = get_factors(num);
        for &v in &curr {
            if freq[v as usize] == 0 {
                distinct += 1;
            }
            freq[v as usize] += 1;
        }
        seen.push(curr);
        while distinct > k && left <= right {
            for &p in &seen[left] {
                freq[p as usize] -= 1;
                if freq[p as usize] == 0 {
                    distinct -= 1;
                }
            }
            left += 1;
        }
        res = res.max(1 + right - left);
    }
    res as i32
}

fn get_factors(mut num: i32) -> Vec<i32> {
    let mut curr = vec![];
    while num > 1 {
        let p = SPF[num as usize];
        curr.push(p);
        while num % p == 0 {
            num /= p;
        }
    }
    curr
}

const M: usize = 100_000;
static SPF: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut spf: Vec<_> = (0..=M as i32).collect();
    for p in 2..=M.isqrt() {
        if spf[p as usize] == p as i32 {
            for val in (p * p..=M).step_by(p) {
                spf[val as usize] = p as i32;
            }
        }
    }
    spf
});

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(longest_subarray(&[7, 6, 10, 12, 11], 3), 3);
    }

    #[test]
    fn test() {}
}
