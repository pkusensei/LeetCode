mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_digit(mut k: i64) -> i32 {
    let mut len = 1;
    loop {
        let curr = i64::from(len) * 9 * 10_i64.pow(len - 1);
        if curr >= k {
            break;
        }
        k -= curr;
        len += 1;
    }
    k -= 1;
    let num = 10_i64.pow(len - 1) + k / i64::from(len);
    let k = k % i64::from(len);
    let d = num.to_string().into_bytes()[k as usize] - b'0';
    if k < i64::from(len - 1) || (num / 10) & 1 == 0 {
        d.into()
    } else {
        (9 - d).into()
    }
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
