mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(nums: &[i32], x: i32) -> i64 {
    let n = nums.len();
    let prev = i64::from(nums[0]);
    prev + dfs(nums, x.into(), 1, prev & 1, &mut vec![[-1, -1]; n])
}

fn dfs(nums: &[i32], x: i64, idx: usize, par: i64, memo: &mut [[i64; 2]]) -> i64 {
    if idx >= nums.len() {
        return 0;
    }
    if memo[idx][par as usize] > -1 {
        return memo[idx][par as usize];
    }
    let skip = dfs(nums, x, 1 + idx, par, memo);
    let val = i64::from(nums[idx]);
    let take = val + dfs(nums, x, 1 + idx, val & 1, memo) - x * ((val & 1) ^ par);
    memo[idx][par as usize] = skip.max(take);
    memo[idx][par as usize]
}

pub fn odd_even_dp(nums: &[i32], x: i32) -> i64 {
    let x = i64::from(x);
    let [mut dp0, mut dp1] = [0_i64, 0];
    for &val in nums.iter().rev() {
        let val = i64::from(val);
        let par = val & 1;
        if par == 0 {
            dp0 = (dp0 + val).max(dp1 + val - x);
        } else {
            dp1 = (dp1 + val).max(dp0 + val - x);
        }
    }
    if nums[0] & 1 == 1 { dp1 } else { dp0 }
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
        assert_eq!(max_score(&[2, 3, 6, 1, 9, 2], 5), 13);
        assert_eq!(max_score(&[2, 4, 6, 8], 3), 20);

        assert_eq!(odd_even_dp(&[2, 3, 6, 1, 9, 2], 5), 13);
        assert_eq!(odd_even_dp(&[2, 4, 6, 8], 3), 20);
    }

    #[test]
    fn test() {
        assert_eq!(
            max_score(
                &[
                    9, 58, 17, 54, 91, 90, 32, 6, 13, 67, 24, 80, 8, 56, 29, 66, 85, 38, 45, 13,
                    20, 73, 16, 98, 28, 56, 23, 2, 47, 85, 11, 97, 72, 2, 28, 52, 33
                ],
                90
            ),
            886
        );
        assert_eq!(
            odd_even_dp(
                &[
                    9, 58, 17, 54, 91, 90, 32, 6, 13, 67, 24, 80, 8, 56, 29, 66, 85, 38, 45, 13,
                    20, 73, 16, 98, 28, 56, 23, 2, 47, 85, 11, 97, 72, 2, 28, 52, 33
                ],
                90
            ),
            886
        );
    }
}
