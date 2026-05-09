mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_flips(s: String) -> i32 {
    let n = s.len();
    if n < 3 {
        return 0;
    }
    let [mut zero, mut one] = [0, 0];
    for &b in s.as_bytes()[1..n - 1].iter() {
        if b == b'0' { zero += 1 } else { one += 1 }
    }
    let first_one = s.starts_with('1');
    let last_one = s.ends_with('1');
    let all_zero = one + i32::from(first_one) + i32::from(last_one);
    let all_one = zero + i32::from(!first_one) + i32::from(!last_one);
    let single_one = (one + i32::from(first_one) + i32::from(last_one) - 1).max(0);
    let se = i32::from(!first_one) + i32::from(!last_one) + one;
    all_zero.min(all_one).min(single_one).min(se)
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
