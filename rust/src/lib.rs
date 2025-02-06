mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn min_changes(nums: &[i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut freq = vec![vec![0; 1024]; k];
    let mut nums_at_pos = vec![HashSet::new(); k];
    for (idx, &num) in nums.iter().enumerate() {
        let pos = idx % k;
        freq[pos][num as usize] += 1;
        nums_at_pos[pos].insert(num);
    }
    let mut dp = vec![vec![1 + n as i32; 1024]; k];
    let mut prev_best = 0;
    for pos in 0..k {
        let count = (n / k) as i32 + i32::from(n % k > pos);
        let mut curr_best = 1 + n as i32;
        // iterate all possible end vals of xor
        for val in 0..1024 {
            if pos == 0 {
                dp[pos][val] = count - freq[pos][val];
            } else {
                // To make all nums on this pos to num
                for &num in nums_at_pos[pos].iter() {
                    // previous dp res + count of changes needed
                    dp[pos][val] = dp[pos][val]
                        .min(dp[pos - 1][val ^ num as usize] + count - freq[pos][num as usize]);
                }
                // For val absent in nums_at_pos[pos]
                dp[pos][val] = dp[pos][val].min(prev_best + count);
            }
            curr_best = curr_best.min(dp[pos][val]);
        }
        prev_best = curr_best;
    }
    dp[k - 1][0]
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
        assert_eq!(min_changes(&[1, 2, 0, 3, 0], 1), 3);
        assert_eq!(min_changes(&[3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
        assert_eq!(min_changes(&[1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
    }

    #[test]
    fn test() {}
}
