mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn shadow_pairs(mut nums: Vec<i32>) -> i32 {
    let sorted = nums.iter().copied().sorted_unstable().dedup().collect_vec();
    for num in nums.iter_mut() {
        let x = *num;
        let i = sorted.partition_point(|&v| v < x);
        *num = i as i32;
    }
    dfs(&nums, 0, sorted.len() - 1)
}

fn dfs(nums: &[i32], left: usize, right: usize) -> i32 {
    if nums.len() <= 1 || left == right {
        return 0;
    }
    // inc stack
    // loose-dec stack
    let [mut inc_st, mut dec_st] = [const { vec![] }; 2];
    let [mut low, mut high] = [const { vec![] }; 2];
    let mid = left.midpoint(right);
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        if num <= mid as i32 {
            while let Some(&top) = dec_st.last()
                && nums[top] < num
            {
                // [top] < num; top cannot be [i]
                dec_st.pop();
            }
            dec_st.push(idx);
            low.push(num);
        } else {
            while let Some(&top) = inc_st.last()
                && nums[top] >= num
            {
                // Pop all big [top]
                // Effectively find closest [k] < [j]
                inc_st.pop();
            }
            res += dec_st.len() as i32;
            if let Some(&top) = inc_st.last() {
                res -= dec_st.partition_point(|&v| v < top) as i32;
            }
            inc_st.push(idx);
            high.push(num);
        }
    }
    res + dfs(&low, left, mid) + dfs(&high, 1 + mid, right)
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
