mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_consistent(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut left_end = vec![-1; n];
    let mut st: Vec<usize> = vec![];
    for (idx, &num) in nums.iter().enumerate() {
        while let Some(&top) = st.last()
            && nums[top] > num
            && nums[top] & num == num
        {
            st.pop();
        }
        if let Some(&v) = st.last() {
            left_end[idx] = v as i64;
        }
        st.push(idx);
    }
    st.clear();
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate().rev() {
        while let Some(&top) = st.last()
            && nums[top] & num == num
        {
            st.pop();
        }
        let right = st.last().map(|&v| v).unwrap_or(n) as i64;
        st.push(idx);
        let i = idx as i64;
        res += (i - left_end[idx]) * (right - i);
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
