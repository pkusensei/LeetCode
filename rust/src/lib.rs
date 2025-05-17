mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sort_colors(nums: &mut Vec<i32>) {
    let n = nums.len();
    if n == 1 {
        return;
    }
    let [mut left, mut right] = [0, n - 1];
    let mut i = 0;
    while i <= right {
        if nums[i] == 0 {
            nums.swap(left, i);
            left += 1;
            i += 1;
        } else if nums[i] == 2 {
            nums.swap(right, i);
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            i += 1
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
        let mut v = vec![2, 0, 1];
        sort_colors(&mut v);
        assert_eq!(v, [0, 1, 2]);
    }

    #[test]
    fn test() {
        let mut v = vec![2, 2];
        sort_colors(&mut v);
        assert_eq!(v, [2, 2]);
    }
}
