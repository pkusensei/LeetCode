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

fn rolling_subarr(nums: &[i32]) -> i32 {
    fn f_acc(acc: i32, val: i32) -> i32 {
        todo!() // stub
    }
    fn f_filter(val: i32) -> bool {
        todo!() // stub
    }

    let mut res = 0;
    // All subarrs ending on previous element
    let mut prev = HashMap::new();
    for &num in nums.iter() {
        let mut curr = HashMap::from([(num, 1)]);
        if f_filter(num) {
            res += 1; // check subarr [num]
        }
        for (v, f) in prev {
            let val = f_acc(v, num);
            *curr.entry(val).or_insert(0) += f;
            if f_filter(val) {
                res += f;
            }
        }
        prev = curr;
    }
    res
}

pub fn count_good_subarrays(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    let mut prev_good = HashMap::new();
    let mut prev_bad = HashMap::new();
    let mut res = 0;
    for &num in nums.iter() {
        res += 1; // this num
        let mut curr_good = HashMap::from([(num, 1)]);
        let mut curr_bad = HashMap::new();
        for (v, f) in prev_good {
            let val = v | num;
            if val == v || val == num {
                *curr_good.entry(val).or_insert(0) += f;
                res += f;
            } else {
                *curr_bad.entry(val).or_insert(0) += f;
            }
        }
        for (v, f) in prev_bad {
            let val = v | num;
            if val == num {
                res += f;
                *curr_good.entry(val).or_insert(0) += f;
            } else {
                *curr_bad.entry(val).or_insert(0) += f;
            }
        }
        prev_good = curr_good;
        prev_bad = curr_bad;
    }
    res
}

pub fn with_stack(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut st = vec![];
    let mut right_greater = vec![n; n];
    for (i, &num) in nums.iter().enumerate() {
        // Try extend subarr where [top] is the max after bit_or
        // [top] < (num|[top]) means this subarr must end
        // Includes equal case
        while let Some(&top) = st.last()
            && nums[top] < num | nums[top]
        {
            st.pop();
            right_greater[top] = i;
        }
        st.push(i);
    }
    st.clear();
    let mut left_greater = vec![None; n];
    for (i, &num) in nums.iter().enumerate().rev() {
        // Excludes equal case
        while let Some(&top) = st.last()
            && (nums[top] < num | nums[top] || num == nums[top])
        {
            st.pop();
            left_greater[top] = Some(i);
        }
        st.push(i);
    }
    let mut res = 0;
    for (i, (left, right)) in left_greater.iter().zip(right_greater).enumerate() {
        let left = left.map(|v| v as i64).unwrap_or(-1);
        res += (right - i) as i64 * (i as i64 - left);
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
    fn basics() {}

    #[test]
    fn test() {}
}
