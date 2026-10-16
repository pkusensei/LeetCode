mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let [
        mut res,
        mut pos_no_del,
        mut neg_no_del,
        mut pos_del,
        mut neg_del,
    ] = [i64::MIN >> 1; 5];
    for &num in nums.iter() {
        let num = i64::from(num);
        let new_pos_del = pos_no_del.max(num + neg_del);
        neg_del = neg_no_del.max(pos_del - num);
        pos_del = new_pos_del;
        let new_pos_no_del = num.max(neg_no_del + num);
        neg_no_del = pos_no_del - num;
        pos_no_del = new_pos_no_del;
        res = res
            .max(pos_no_del)
            .max(neg_no_del)
            .max(pos_del)
            .max(neg_del);
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
