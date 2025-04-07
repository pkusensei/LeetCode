mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_increasing_groups(mut usage_limits: Vec<i32>) -> i32 {
    usage_limits.sort_unstable();
    let mut prefix = 0;
    let mut res = 0;
    for &num in usage_limits.iter() {
        prefix += i64::from(num);
        if prefix > res {
            res += 1; // enough to form new group
            prefix -= res;
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
    fn basics() {
        assert_eq!(max_increasing_groups(vec![1, 2, 5]), 3);
        assert_eq!(max_increasing_groups(vec![2, 1, 2]), 2);
        assert_eq!(max_increasing_groups(vec![1, 1]), 1);
    }

    #[test]
    fn test() {}
}
