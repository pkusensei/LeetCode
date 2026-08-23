mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_game(num: String) -> bool {
    let n = num.len();
    let mut sum = 0;
    let [mut q1, mut q2] = [0, 0];
    for (i, b) in num.bytes().enumerate() {
        if i < n / 2 {
            if b == b'?' {
                q1 += 1
            } else {
                sum += i32::from(b - b'0')
            }
        } else {
            if b == b'?' {
                q2 += 1
            } else {
                sum -= i32::from(b - b'0')
            }
        }
    }
    if (q1 + q2) & 1 == 1 {
        return true;
    }
    sum != (q2 - q1) * 9 / 2
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
