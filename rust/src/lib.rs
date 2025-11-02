mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time(d: Vec<i32>, r: Vec<i32>) -> i64 {
    let lcm_ = lcm(r[0].into(), r[1].into());
    let mut left = 1;
    let mut right = i64::MAX >> 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if check(&d, &r, mid, lcm_) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left
}

fn check(d: &[i32], r: &[i32], mid: i64, lcm_: i64) -> bool {
    let [rest1, rest2] = [0, 1].map(|i| mid / i64::from(r[i]));
    let common = mid - rest1 - rest2 + mid / lcm_;
    let [h1, h2] = [rest1, rest2].map(|v| mid - v - common);
    let d1 = i64::from(d[0]) - h1;
    let d2 = i64::from(d[1]) - h2;
    d1.max(0) + d2.max(0) <= common
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
