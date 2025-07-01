mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_coins(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    nums.insert(0, 1);
    let mut res = vec![];
    for c in 1..=n {
        if nums[c] == 0 {
            continue;
        }
        if nums[c] > 1 {
            return vec![];
        }
        res.push(c as i32);
        for s in (c..=n).rev() {
            nums[s] -= nums[s - c];
            if nums[s] < 0 {
                return vec![];
            }
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
        assert_eq!(find_coins(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5]), [2, 4, 6]);
        assert_eq!(find_coins(vec![1, 2, 2, 3, 4]), [1, 2, 5]);
        assert_eq!(find_coins(vec![1, 2, 3, 4, 15]), []);
    }

    #[test]
    fn test() {}
}
