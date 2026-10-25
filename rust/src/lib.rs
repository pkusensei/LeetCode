mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// (1+316)*316/2 = 50086
pub fn count_staircase(nums: &[i32]) -> i32 {
    let n = nums.len();
    let max = *nums.iter().max().unwrap();
    let mut freq = vec![0; 1 + max as usize];
    let mut required = vec![0; 1 + max as usize];
    (0..n)
        .map(|i| f(&nums[i..], &mut freq, &mut required))
        .sum()
}

const LENGTHS: [usize; 316] = {
    let mut v = [0; 316];
    let mut idx = 0;
    while idx < 316 {
        v[idx] = (1 + idx) * (2 + idx) / 2;
        idx += 1;
    }
    v
};

fn f(nums: &[i32], freq: &mut [i32], required: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut distinct = 0;
    let mut matched = 0;
    let mut right = 0;
    for (_i, &num) in nums.iter().enumerate() {
        let val = num as usize;
        if freq[val] == 0 {
            distinct += 1;
            if distinct * (1 + distinct) / 2 > n as i32 {
                break;
            }
            required[val] = distinct;
        }
        freq[val] += 1;
        right = _i;
        if freq[val] > required[val] {
            break;
        }
        if freq[val] == required[val] {
            matched += 1;
        }
        res += i32::from(matched == distinct);
    }
    for &num in nums[..=right].iter() {
        freq[num as usize] = 0;
    }
    res
}

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
        assert_eq!(count_staircase(&[5, 8, 8, 2, 2, 2]), 9);
    }

    #[test]
    fn test() {}
}
