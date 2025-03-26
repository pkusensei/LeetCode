mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut freq = nums.iter().fold(vec![0; n], |mut acc, &v| {
        acc[v as usize] += 1;
        acc
    });
    let mut scores = vec![vec![0; n]; n];
    for right in (0..n).rev() {
        imports(&nums[..=right], k, 0, &mut freq, &mut scores);
        freq[nums[right] as usize] -= 1;
    }
    let mut dp = vec![i32::MAX; 1 + n];
    dp[0] = 0;
    for right in 1..=n {
        for left in 0..right {
            dp[right] = dp[right].min(dp[left] + scores[left][right - 1]);
        }
    }
    dp[n]
}

fn imports(nums: &[i32], k: i32, left: usize, freq: &mut [u16], scores: &mut [Vec<i32>]) -> i32 {
    let n = nums.len();
    if left >= n {
        return k;
    }
    let curr = nums[left] as usize;
    freq[curr] -= 1;
    let mut res = imports(nums, k, 1 + left, freq, scores);
    match freq[curr] {
        1 => res += 2,
        2.. => res += 1,
        _ => (),
    };
    freq[curr] += 1;
    scores[left][n - 1] = res;
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
        assert_eq!(min_cost(&[1, 2, 1, 2, 1, 3, 3], 2), 8);
        assert_eq!(min_cost(&[1, 2, 1, 2, 1], 2), 6);
        assert_eq!(min_cost(&[1, 2, 1, 2, 1], 5), 10);
    }

    #[test]
    fn test() {}
}
