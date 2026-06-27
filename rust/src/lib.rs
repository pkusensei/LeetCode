mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_length(nums: Vec<i32>) -> i32 {
    use itertools::Itertools;
    let freq = nums.iter().copied().counts();
    let mut res = freq
        .get(&1)
        .map(|&v| if v & 1 == 1 { v } else { v - 1 })
        .unwrap_or(1) as i32;
    for (&k, &v) in freq.iter() {
        if k == 1 || v == 1 {
            continue;
        }
        let mut k = k;
        let mut chain = 0;
        while let Some(&v) = freq.get(&k) {
            chain += 1;
            if v > 1 {
                let Some(kk) = k.checked_mul(k) else {
                    break;
                };
                k = kk;
            } else {
                break;
            }
        }
        res = res.max(2 * chain - 1);
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
