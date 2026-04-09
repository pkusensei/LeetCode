mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn xor_after_queries(nums: &[i32], queries: &[[i32; 4]]) -> i32 {
    let n = nums.len();
    let root = 1 + n.isqrt();
    let mut nums: Vec<_> = nums.iter().map(|v| i64::from(*v)).collect();
    // diffs[k] diff array with step size k
    let mut diffs = vec![vec![]; root];
    for q in queries {
        let [l, r, k, v] = q[..] else { unreachable!() };
        let [left, right, k] = [l, r, k].map(|v| v as usize);
        let v = i64::from(v);
        if k < root {
            let group = {
                if diffs[k].is_empty() {
                    diffs[k] = vec![1; n]; // init here to skip never-seen `k`s
                }
                &mut diffs[k]
            };
            // [left] *= v
            group[left] = group[left] * v % M;
            let right = right + k - (right - left) % k;
            // [right] /= v
            if let Some(v_) = group.get_mut(right) {
                *v_ = (*v_) * mod_pow(v, M - 2) % M;
            }
        } else {
            for idx in (left..=right).step_by(k) {
                nums[idx] = nums[idx] * v % M;
            }
        }
    }
    for (k, diff) in diffs.iter().enumerate() {
        if diff.is_empty() {
            continue;
        }
        for start in 0..k {
            let mut prefix = 1;
            for i in (start..n).step_by(k) {
                prefix = prefix * diff[i] % M;
                nums[i] = nums[i] * prefix % M;
            }
        }
    }
    nums.iter().fold(0, |acc, v| acc ^ v) as i32
}

const M: i64 = 1_000_000_007;

const fn mod_pow(b: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % M, exp >> 1)
    } else {
        mod_pow(b * b % M, exp >> 1) * b % M
    }
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
