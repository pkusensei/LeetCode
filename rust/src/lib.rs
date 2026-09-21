mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// count all product(subarr)%k
pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
    let k = k as usize;
    let mut dp = vec![0; k];
    let mut res = vec![0; k];
    for &num in nums.iter() {
        let rem = num as usize % k;
        let mut curr = vec![0; k];
        curr[rem] = 1; // start new subarr
        for (val, f) in dp.iter().enumerate() {
            let rem = val * rem % k;
            curr[rem] += f; // continue previous subarr
        }
        dp = curr;
        for i in 0..k {
            res[i] += dp[i]; // collect subarr ending here
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}
