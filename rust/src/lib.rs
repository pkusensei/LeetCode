mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let prefix = nums.iter().fold(Vec::with_capacity(n), |mut acc, &num| {
        acc.push(i64::from(num) + acc.last().unwrap_or(&0));
        acc
    });
    let mut res = 0;
    let mut curr = u64::MAX;
    for (idx, &left) in prefix.iter().enumerate() {
        let right = prefix[n - 1] - left;
        let aleft = left / (1 + idx as i64);
        let aright = if idx == n - 1 {
            0
        } else {
            right / (n - idx - 1) as i64
        };
        if aleft.abs_diff(aright) < curr {
            curr = aleft.abs_diff(aright);
            res = idx;
        }
    }
    res as i32
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
    fn basics() {}

    #[test]
    fn test() {}
}
