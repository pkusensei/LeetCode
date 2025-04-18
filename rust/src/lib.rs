mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_maximum_length(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut vals = vec![i64::MAX; 1 + n];
    vals[0] = 0;
    let prefix = nums.iter().fold(vec![0], |mut acc, &v| {
        acc.push(i64::from(v) + acc.last().unwrap_or(&0));
        acc
    });
    let mut dp = vec![0; 1 + n];
    let mut prev_idx = vec![0; 2 + n];
    let mut left = 0;
    for right in 1..=n {
        left = left.max(prev_idx[right]);
        dp[right] = 1 + dp[left];
        // Find first v in prefix_sum
        // such that v-prefix[right] >= prefix[right]-prefix[left]
        // i.e sum(right..=v)>=sum(left..=right)
        let i = prefix.partition_point(|&v| v < 2 * prefix[right] - prefix[left]);
        prev_idx[i] = right;
    }
    dp[n]
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
        assert_eq!(find_maximum_length(&[5, 2, 2]), 1);
        assert_eq!(find_maximum_length(&[1, 2, 3, 4]), 4);
        assert_eq!(find_maximum_length(&[4, 3, 2, 6]), 3);
    }

    #[test]
    fn test() {
        assert_eq!(
            find_maximum_length(&[336, 78, 256, 976, 976, 764, 370, 46]),
            4
        );
    }
}
