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

pub fn longest_balanced(s: String) -> i32 {
    let n = s.len();
    let total1 = s.bytes().map(|b| i32::from(b - b'0')).sum::<i32>();
    let total0 = n as i32 - total1;
    let limit = 2 * total0.min(total1); // max possible length
    // (sum, prev_indices)
    let mut prefix = HashMap::from([(0, vec![-1])]);
    let mut sum = 0;
    let mut res = 0;
    // 1 1 0 0 1
    // 1 2 1 0 1
    for (idx, b) in s.bytes().enumerate() {
        sum += if b == b'1' { 1 } else { -1 };
        if let Some(p) = prefix.get(&sum) {
            res = res.max(idx as i32 - p[0]);
        }
        for target in [sum - 2, sum + 2] {
            if let Some(prev) = prefix.get(&target) {
                let i = prev.partition_point(|&v| v < idx as i32 - limit);
                if i < prev.len() {
                    res = res.max(idx as i32 - prev[i]);
                }
            }
        }
        prefix.entry(sum).or_default().push(idx as i32);
    }
    res.min(limit)
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
