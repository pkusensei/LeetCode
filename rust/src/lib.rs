mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut prev = i32::MIN;
    let mut res = 0;
    for &num in &nums {
        if prev >= num + k {
            continue;
        }
        res += 1;
        prev = (prev + 1).max(num - k); // increment by 1 or jump
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
        assert_eq!(max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2), 6);
        assert_eq!(max_distinct_elements(vec![4, 4, 4, 4], 1), 3);
    }

    #[test]
    fn test() {}
}
