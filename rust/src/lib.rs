mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
    let k = k as usize;
    let mut dp = vec![0; k];
    let mut res = vec![0; k];
    for &num in nums.iter() {
        let mut curr = vec![0; k];
        let rem = num as usize % k;
        curr[rem] = 1;
        for i in 0..k {
            let val = rem * i % k;
            curr[val] += dp[i];
        }
        for (v, c) in res.iter_mut().zip(curr.iter()) {
            *v += c;
        }
        dp = curr;
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
    fn basics() {}

    #[test]
    fn test() {}
}
