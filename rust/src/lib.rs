mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    let [a, b, n] = [a, b, n].map(i64::from);
    let lcm_ = lcm(a, b);
    let mut left = 1;
    let mut right = i64::MAX >> 1;
    while left < right {
        let mid = left + (right - left) / 2;
        let c = mid / a + mid / b - mid / lcm_;
        if c < n { left = 1 + mid } else { right = mid }
    }
    (left % 1_000_000_007) as i32
}

const fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

const fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { b } else { gcd(b % a, a) }
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
