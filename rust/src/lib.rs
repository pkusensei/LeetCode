mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_fixed_points(nums: &[i32]) -> i32 {
    let mut arr = vec![];
    for (i, &num) in nums.iter().enumerate() {
        if i as i32 >= num {
            arr.push([i as i32 - num, num]);
        }
    }
    arr.sort_unstable();
    let mut lis = vec![];
    for [_, val] in arr {
        let i = lis.partition_point(|&v| v < val);
        if i == lis.len() {
            lis.push(val);
        } else {
            lis[i] = val
        }
    }
    lis.len() as i32
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
    fn basics() {
        assert_eq!(max_fixed_points(&[3, 1, 2]), 2);
    }

    #[test]
    fn test() {
        assert_eq!(max_fixed_points(&[1, 1, 0]), 1);
        assert_eq!(max_fixed_points(&[2, 0, 0]), 1);
    }
}
