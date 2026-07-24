mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap();
    let n = 1_usize << (1 + max.ilog2());
    let mut two = vec![false; n];
    for (i, a) in nums.iter().enumerate() {
        for b in &nums[i..] {
            two[(a ^ b) as usize] = true;
        }
    }
    let mut three = vec![false; n];
    for (val, &flag) in two.iter().enumerate() {
        if flag {
            for &num in nums.iter() {
                three[val ^ num as usize] = true;
            }
        }
    }
    three.iter().filter(|&&v| v).count() as i32
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
    fn basics() {}

    #[test]
    fn test() {}
}
