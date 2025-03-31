mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_prefix_score(nums: &[i32]) -> Vec<i64> {
    let mut res = vec![];
    let mut max = 0;
    let mut prefix = 0;
    for num in nums.iter().map(|&v| i64::from(v)) {
        max = max.max(num);
        prefix += num + max;
        res.push(prefix);
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
        assert_eq!(find_prefix_score(&[2, 3, 7, 5, 10]), [4, 10, 24, 36, 56]);
        assert_eq!(
            find_prefix_score(&[1, 1, 2, 4, 8, 16]),
            [2, 4, 8, 16, 32, 64]
        );
    }

    #[test]
    fn test() {}
}
