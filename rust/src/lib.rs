mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subsequence_sum_after_capping(mut nums: Vec<i32>, k: i32) -> Vec<bool> {
    let (n, k) = (nums.len(), k as usize);
    nums.sort_unstable();
    // true if sum `i` is reachable
    let mut sums = vec![false; 1 + k];
    sums[0] = true;
    let mut res = Vec::with_capacity(n);
    let mut idx = 0;
    'outer: for cap in 1..=n {
        // `val` is "uncapped". It can be used to build to `k`
        while let Some(&val) = nums.get(idx)
            && (val as usize) < cap
        {
            // Standard subset sum DP update (knapsack)
            // Go backwards to avoid using same element multiple times
            for i in (val as usize..=k).rev() {
                if sums[i - val as usize] {
                    sums[i as usize] = true;
                }
            }
            idx += 1;
        }
        let bigger = n - idx; // These are seen as `cap`
        for i in (0..=bigger).take_while(|i| i * cap <= k as usize) {
            if sums[k as usize - i * cap] {
                res.push(true);
                continue 'outer;
            }
        }
        res.push(false);
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
        assert_eq!(
            subsequence_sum_after_capping(vec![4, 3, 2, 4], 5),
            [false, false, true, true]
        );
        assert_eq!(
            subsequence_sum_after_capping(vec![1, 2, 3, 4, 5], 3),
            [true; 5]
        );
    }

    #[test]
    fn test() {}
}
