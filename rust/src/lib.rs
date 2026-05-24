mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_jumps(arr: &[i32], d: i32) -> i32 {
    use itertools::Itertools;
    use std::cmp::Reverse;

    let n = arr.len();
    let mut suf_greater = (0..n).map(|v| (1 + v + d as usize).min(n)).collect_vec();
    let mut st: Vec<usize> = vec![];
    for (idx, &num) in arr.iter().enumerate() {
        while let Some(&top) = st.last()
            && arr[top] <= num
        {
            st.pop();
            suf_greater[top] = suf_greater[top].min(idx);
        }
        st.push(idx);
    }
    st.clear();
    let mut pref_greater = (0..n).map(|v| v.saturating_sub(d as usize)).collect_vec();
    for (idx, &num) in arr.iter().enumerate().rev() {
        while let Some(&top) = st.last()
            && arr[top] <= num
        {
            st.pop();
            pref_greater[top] = pref_greater[top].max(1 + idx);
        }
        st.push(idx);
    }
    let nums = (0..n)
        .sorted_unstable_by_key(|i| Reverse(arr[*i]))
        .collect_vec();
    let mut dp = vec![1; n];
    let mut res = 1;
    for idx in nums {
        let left = pref_greater[idx];
        let right = suf_greater[idx];
        for i in left..right {
            if i == idx {
                continue;
            }
            dp[i] = dp[i].max(1 + dp[idx]);
            res = res.max(dp[i]);
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
        assert_eq!(max_jumps(&[7, 6, 5, 4, 3, 2, 1], 1), 7);
    }

    #[test]
    fn test() {}
}
