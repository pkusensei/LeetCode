mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn capture_forts(forts: &[i32]) -> i32 {
    let mut neg_one = -1;
    let mut one = -1;
    let mut res = 0;
    for (idx, &num) in (0..).zip(forts.iter()) {
        if num == -1 {
            if one < neg_one {
                one = -1;
            }
            neg_one = idx;
            if one > -1 {
                res = res.max(neg_one - one - 1);
            }
        }
        if num == 1 {
            if neg_one < one {
                neg_one = -1;
            }
            one = idx;
            if neg_one > -1 {
                res = res.max(one - neg_one - 1);
            }
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
        assert_eq!(capture_forts(&[1, 0, 0, -1, 0, 0, 0, 0, 1]), 4);
        assert_eq!(capture_forts(&[0, 0, 1, -1]), 0);
    }

    #[test]
    fn test() {
        assert_eq!(capture_forts(&[1, 0, 0, -1, 0, 0, -1, 0, 0, 1]), 2);
    }
}
