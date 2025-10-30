mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut res = 0;
    let mut skip = 0;
    let mut prev = 0;
    for ch in nums.chunk_by(|a, b| a == b) {
        let val = ch[0] * ch.len() as i32;
        let temp = res;
        if 1 + prev < ch[0] {
            res += val;
        } else {
            res = res.max(skip + val);
        }
        skip = temp;
        prev = ch[0];
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
        assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }

    #[test]
    fn test() {
        assert_eq!(delete_and_earn(vec![1, 6, 3, 3, 8, 4, 8, 10, 1, 3]), 43);
    }
}
