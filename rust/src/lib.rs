mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut curr = 1;
    let mut prev = -1;
    let mut res = vec![];
    for (i, &num) in nums.iter().enumerate() {
        if num == prev + 1 {
            curr += 1;
        } else {
            curr = 1;
        }
        prev = num;
        if i >= k as usize - 1 {
            res.push(if curr >= k { num } else { -1 });
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
