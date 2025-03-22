mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn best_closing_time(customers: String) -> i32 {
    let mut score = 0;
    let mut curr = 0;
    let mut res = -1;
    for (idx, b) in (0..).zip(customers.bytes()) {
        if b == b'Y' {
            curr += 1
        } else {
            curr -= 1
        }
        if curr > score {
            score = curr;
            res = idx;
        }
    }
    1 + res as i32
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
