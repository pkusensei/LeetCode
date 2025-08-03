mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_trionic(nums: &[i32]) -> i64 {
    const MIN: i64 = i64::MIN / 2;
    let n = nums.len();
    // start, inc, dec, inc
    let [mut dp0, mut dp1, mut dp2, mut dp3] = [0; 4].map(|_| vec![MIN; n]);
    dp0[0] = i64::from(nums[0]);
    for i in 1..n {
        let val = i64::from(nums[i]);
        match nums[i - 1].cmp(&nums[i]) {
            std::cmp::Ordering::Less => {
                // start from [i-1] or [i]
                dp0[i] = val.max(dp0[i - 1] + val);
                // continue inc or initiate anew
                dp1[i] = val + dp1[i - 1].max(dp0[i - 1]);
                // Similarly, continue inc or initiate anew from dec
                dp3[i] = val + dp3[i - 1].max(dp2[i - 1]);
            }
            std::cmp::Ordering::Equal => {
                dp0[i] = val; // Have to start anew
            }
            std::cmp::Ordering::Greater => {
                // Start anew since subarr always begins with inc order
                dp0[i] = val;
                dp2[i] = val + dp1[i - 1].max(dp2[i - 1]);
            }
        }
    }
    *dp3[3..].iter().max().unwrap()
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
        assert_eq!(max_sum_trionic(&[0, -2, -1, -3, 0, 2, -1]), -4);
        assert_eq!(max_sum_trionic(&[1, 4, 2, 7]), 14);
    }

    #[test]
    fn test() {
        assert_eq!(max_sum_trionic(&[35, 941, 281, 713, -160, 996]), 1970);
        assert_eq!(
            max_sum_trionic(&[-754, 167, -172, 202, 735, -941, 992]),
            988
        );
    }
}
