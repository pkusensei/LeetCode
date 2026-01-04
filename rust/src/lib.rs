mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_merge_cost(lists: &[&[i32]]) -> i64 {
    use itertools::chain;

    let n = lists.len();
    let full_mask = 1 << n; // well, 1+full_mask
    // For convenience, store all intermediate vecs
    let mut arrs = vec![vec![]; full_mask];
    // Record all medians
    let mut meds = vec![0; full_mask];
    for mask in 1..full_mask {
        // Since we loop from small to large mask
        // Each loop we only scoop up only one arr, i.e lists[log2(lsb)]
        let lsb = mask & mask.wrapping_neg();
        let mut curr: Vec<i32> =
            chain!(arrs[mask ^ lsb].iter(), lists[lsb.ilog2() as usize].iter())
                .copied()
                .collect();
        let len = curr.len();
        let (_, med, _) = curr.select_nth_unstable((len - 1) / 2);
        meds[mask] = *med;
        arrs[mask] = curr;
    }
    let mut dp = vec![i64::MAX >> 2; full_mask];
    dp[0] = 0;
    // SoS dp
    for mask in 1..full_mask {
        if mask.count_ones() == 1 {
            dp[mask] = 0; // merge with itself
            continue;
        }
        for subset in 1..mask {
            if mask | subset == mask {
                // Consider all valid subsets/submasks
                let other = mask ^ subset;
                let cost = (arrs[subset].len() + arrs[other].len()) as i64
                    + i64::from(meds[subset].abs_diff(meds[other]))
                    + dp[subset]
                    + dp[other];
                dp[mask] = dp[mask].min(cost);
            }
        }
    }
    dp[full_mask - 1]
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
        assert_eq!(min_merge_cost(&[&[1, 3, 5], &[2, 4], &[6, 7, 8]]), 18);
        assert_eq!(min_merge_cost(&[&[1, 1, 5], &[1, 4, 7, 8]]), 10);
        assert_eq!(min_merge_cost(&[&[1], &[3]]), 4);
    }

    #[test]
    fn test() {}
}
