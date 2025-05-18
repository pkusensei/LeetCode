mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_possible_score(mut start: Vec<i32>, d: i32) -> i32 {
    start.sort_unstable();
    let mut left = 0;
    let mut right = i32::MAX - 1;
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if chech(&start, d, mid) {
            left = mid
        } else {
            right = mid - 1
        }
    }
    left
}

fn chech(nums: &[i32], d: i32, mid: i32) -> bool {
    let mut prev = nums[0];
    for &num in &nums[1..] {
        let curr = (prev + mid).max(num);
        if curr > num + d {
            return false;
        }
        prev = curr;
    }
    true
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
        assert_eq!(max_possible_score(vec![2, 6, 13, 13], 5), 5);
        assert_eq!(max_possible_score(vec![6, 0, 3], 2), 4);
    }

    #[test]
    fn test() {
        assert_eq!(
            max_possible_score(vec![1000000000, 0], 1000000000),
            2000000000
        );
    }
}
