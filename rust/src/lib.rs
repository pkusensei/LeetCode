mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = Vec::with_capacity(n);
    let mut right = nums.partition_point(|&v| v < 0);
    if right == 0 {
        return nums.iter().map(|v| v.pow(2)).collect();
    }
    let mut left = right - 1;
    while right < n {
        if nums[left].abs() < nums[right] {
            res.push(nums[left].pow(2));
            let Some(v) = left.checked_sub(1) else {
                left = n;
                break;
            };
            left = v;
        } else {
            res.push(nums[right].pow(2));
            right += 1;
        }
    }
    if left < n {
        res.extend(nums[..=left].iter().rev().map(|v| v.pow(2)));
    }
    res.extend(nums[right..].iter().map(|v| v.pow(2)));
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
