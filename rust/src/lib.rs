mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(s: &str, t: &str, flip_cost: i32, swap_cost: i32, cross_cost: i32) -> i64 {
    if s == t {
        return 0;
    }
    let [mut one_zero, mut zero_one] = [0, 0];
    for (b1, b2) in s.bytes().zip(t.bytes()) {
        match [b1, b2] {
            [b'1', b'0'] => one_zero += 1,
            [b'0', b'1'] => zero_one += 1,
            _ => (),
        }
    }
    let pair = one_zero.min(zero_one);
    let mut res = pair * i64::from(swap_cost.min(2 * flip_cost));
    one_zero -= pair;
    zero_one -= pair;
    res += (one_zero - zero_one).abs() / 2 * i64::from((swap_cost + cross_cost).min(2 * flip_cost));
    res += ((one_zero - zero_one).abs() & 1) * i64::from(flip_cost);
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
        assert_eq!(minimum_cost("01000", "10111", 10, 2, 2), 16);
        assert_eq!(minimum_cost("001", "110", 2, 100, 100), 6);
    }

    #[test]
    fn test() {}
}
