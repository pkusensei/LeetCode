mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn xor_after_queries(mut nums: Vec<i32>, queries: &[[i32; 4]]) -> i32 {
    const M: i64 = 1_000_000_007;
    let n = nums.len();
    let root = 1 + n.isqrt();
    let mut groups = vec![vec![1; n]; root];
    for q in queries.iter() {
        let [left, right, k] = [0, 1, 2].map(|i| q[i] as usize);
        if k < root {
            let diff = &mut groups[k];
            // diff[left] *= q[3]
            diff[left] = diff[left] * i64::from(q[3]) % M;
            let r2 = right + (k - (right - left) % k);
            if r2 < n {
                // diff[r2] /= q[3]
                diff[r2] = diff[r2] * mod_pow(q[3].into(), M - 2, M) % M;
            }
        } else {
            for idx in (left..=right).step_by(k) {
                let val = i64::from(nums[idx]) * i64::from(q[3]) % M;
                nums[idx] = val as i32;
            }
        }
    }
    for (k, row) in groups.iter().enumerate() {
        for rem in 0..k {
            let mut diff = 1;
            for idx in (rem..n).step_by(k) {
                diff = diff * row[idx] % M;
                let val = i64::from(nums[idx]) * diff % M;
                nums[idx] = val as i32;
            }
        }
    }
    nums.iter().fold(0, |acc, v| acc ^ v)
}

const fn mod_pow(b: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % m, exp >> 1, m)
    } else {
        mod_pow(b * b % m, exp >> 1, m) * b % m
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
    fn basics() {
        assert_eq!(xor_after_queries(vec![1, 1, 1], &[[0, 2, 1, 4]]), 4);
        assert_eq!(
            xor_after_queries(vec![2, 3, 1, 5, 4], &[[1, 4, 2, 3], [0, 2, 1, 2]]),
            31
        )
    }

    #[test]
    fn test() {}
}
