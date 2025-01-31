mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ways_to_split(nums: &[i32]) -> i32 {
    let n = nums.len();
    let prefix = nums.iter().fold(Vec::with_capacity(n), |mut acc, &num| {
        acc.push(num + acc.last().unwrap_or(&0));
        acc
    });
    let [mut right1, mut right2] = [0, 0];
    let mut res = 0;
    for left in 0..n - 2 {
        right1 = (1 + left).max(right1);
        while right1 < n - 1 && prefix[right1] < 2 * prefix[left] {
            right1 += 1;
        }
        right2 = right2.max(right1);
        while right2 < n - 1 && prefix[n - 1] - prefix[right2] >= prefix[right2] - prefix[left] {
            right2 += 1;
        }
        res += right2 - right1;
        res %= 1_000_000_007;
    }
    res as _
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(ways_to_split(&[1, 1, 1]), 1);
        assert_eq!(ways_to_split(&[1, 2, 2, 2, 5, 0]), 3);
        assert_eq!(ways_to_split(&[3, 2, 1]), 0);
    }

    #[test]
    fn test() {}
}
