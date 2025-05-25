mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut pre_gcd = Vec::with_capacity(n);
    let mut pre_lcm = Vec::with_capacity(n);
    let mut gcd_ = i64::from(nums[0]);
    let mut lcm_ = gcd_;
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        gcd_ = gcd(num.into(), gcd_);
        lcm_ = lcm(num.into(), lcm_);
        pre_gcd.push(gcd_);
        pre_lcm.push(lcm_);
        if idx == n - 2 {
            res = res.max(gcd_ * lcm_);
        }
    }
    res = res.max(lcm_ * gcd_);
    let mut suf_gcd = Vec::with_capacity(n);
    let mut suf_lcm = Vec::with_capacity(n);
    gcd_ = i64::from(nums[n - 1]);
    lcm_ = gcd_;
    for (idx, &num) in nums.iter().rev().enumerate() {
        gcd_ = gcd(num.into(), gcd_);
        lcm_ = lcm(num.into(), lcm_);
        suf_gcd.push(gcd_);
        suf_lcm.push(lcm_);
        if idx == n - 2 {
            res = res.max(gcd_ * lcm_);
        }
    }
    suf_gcd.reverse();
    suf_lcm.reverse();
    res = res.max(lcm_ * gcd_);
    for idx in 1..n - 1 {
        let left_gcd = pre_gcd[idx - 1];
        let left_lcm = pre_lcm[idx - 1];
        let right_gcd = suf_gcd[1 + idx];
        let right_lcm = suf_lcm[1 + idx];
        gcd_ = gcd(left_gcd, right_gcd);
        lcm_ = lcm(left_lcm, right_lcm);
        res = res.max(lcm_ * gcd_);
    }
    res
}

const fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { b } else { gcd(b % a, a) }
}

const fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
