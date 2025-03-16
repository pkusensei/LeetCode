mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let mut res = 0;
    for &num in nums.iter() {
        if nums.binary_search(&(num + diff)).is_ok()
            && nums.binary_search(&(num + diff * 2)).is_ok()
        {
            res += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
