mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

//  i1 < i2 < i3 < i4
// [i1]<[i3]<[i2]<[i4]
pub fn count_quadruplets(nums: &[i32]) -> i64 {
    let n = nums.len();
    // track 132 triplets in dp[i2]
    let mut count = vec![0; n];
    let mut res = 0;
    for x in 2..n {
        let mut prev_small_pairs = 0;
        for y in 0..x {
            if nums[x] > nums[y] {
                // [x]>[y] => [i3]>[i1]
                // This count is saved for a potential [i2]
                prev_small_pairs += 1;
                // !! semamtics of (x, y) changed !!
                // [x]>[y] => [i4]>[i2]
                // Found valid [i4], accumulate result
                res += count[y];
            } else if nums[x] < nums[y] {
                // [x]<[y] => [i3]<[i2]
                // record the value in dp[i2]
                count[y] += prev_small_pairs;
            }
        }
    }
    res
}

pub fn with_prefix_sums(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut prefix = vec![vec![0; n]; n];
    for i2 in 1..n {
        for i3 in 1 + i2..n {
            // Count all [i1]<[i3]
            prefix[i2][i3] = prefix[i2 - 1][i3] + i64::from(nums[i2 - 1] < nums[i3]);
        }
    }
    let mut suffix = vec![vec![0; n]; n];
    for i3 in (0..n - 1).rev() {
        for i2 in (0..i3).rev() {
            // Count all [i2]<[i4]
            suffix[i3][i2] = suffix[1 + i3][i2] + i64::from(nums[i2] < nums[1 + i3]);
        }
    }
    let mut res = 0;
    for i2 in 1..n - 2 {
        for i3 in 1 + i2..n - 1 {
            if nums[i2] > nums[i3] {
                res += prefix[i2][i3] * suffix[i3][i2];
            }
        }
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
        assert_eq!(count_quadruplets(&[1, 3, 2, 4, 5]), 2);
        assert_eq!(count_quadruplets(&[1, 2, 3, 4]), 0);

        assert_eq!(with_prefix_sums(&[1, 3, 2, 4, 5]), 2);
        assert_eq!(with_prefix_sums(&[1, 2, 3, 4]), 0);
    }

    #[test]
    fn test() {}
}
