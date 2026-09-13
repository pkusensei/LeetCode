mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

// i < k < j
// [i] < [j] && [i] <= [k]
pub fn shadow_pairs(nums: Vec<i32>) -> i64 {
    let mut st = vec![];
    let mut freq = HashMap::new();
    let mut res = 0;
    for (i, &num) in nums.iter().enumerate() {
        while let Some(&top) = st.last()
            && nums[top] > num
        {
            st.pop();
            *freq.entry(nums[top]).or_insert(0) -= 1;
        }
        res += st.len() as i64 - freq.get(&num).unwrap_or(&0);
        *freq.entry(num).or_insert(0) += 1;
        st.push(i);
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
