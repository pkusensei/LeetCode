mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subset_xor_sum(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, num| acc | num) << (nums.len() - 1)
    // let mut res = 0;
    // dfs(nums, 0, &mut res);
    // res
}

fn dfs(nums: &[i32], curr: i32, res: &mut i32) {
    match nums {
        [] => *res += curr,
        [head, tail @ ..] => {
            dfs(tail, curr, res);
            dfs(tail, curr ^ *head, res);
        }
    }
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
        assert_eq!(subset_xor_sum(&[1, 3]), 6);
        assert_eq!(subset_xor_sum(&[5, 1, 6]), 28);
        assert_eq!(subset_xor_sum(&[3, 4, 5, 6, 7, 8]), 480);
    }

    #[test]
    fn test() {}
}
