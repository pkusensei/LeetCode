mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_palindrome(queries: &[i32], int_length: i32) -> Vec<i64> {
    let base = 10i32.pow((int_length as u32 - 1) / 2);
    let mut res = vec![];
    for &q in queries.iter() {
        let mut half = q - 1 + base;
        let mut digits = vec![];
        while half > 0 {
            digits.push(half % 10);
            half /= 10;
        }
        digits.reverse();
        let mut right = digits.clone();
        if int_length & 1 == 1 {
            right.pop();
        }
        right.reverse();
        if right.len() + digits.len() != int_length as usize {
            res.push(-1);
        } else {
            let num = digits
                .into_iter()
                .chain(right)
                .fold(0, |acc, d| 10 * acc + i64::from(d));
            res.push(num);
        }
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
    fn basics() {
        assert_eq!(kth_palindrome(&[2, 4, 6], 4), [1111, 1331, 1551]);
        assert_eq!(
            kth_palindrome(&[1, 2, 3, 4, 5, 90], 3),
            [101, 111, 121, 131, 141, 999]
        );
    }

    #[test]
    fn test() {}
}
