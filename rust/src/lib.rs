mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn good_days_to_rob_bank(security: &[i32], time: i32) -> Vec<i32> {
    let n = security.len();
    let mut decs = Vec::with_capacity(n);
    decs.push(0);
    for i in 1..n {
        if security[i - 1] >= security[i] {
            decs.push(1 + decs[i - 1]);
        } else {
            decs.push(0);
        }
    }
    let mut incs = Vec::with_capacity(n);
    incs.push(0);
    for i in (0..n - 1).rev() {
        if security[i] <= security[1 + i] {
            incs.push(1 + incs.last().unwrap_or(&0));
        } else {
            incs.push(0);
        }
    }
    incs.reverse();
    (0..)
        .zip(decs.into_iter().zip(incs))
        .filter_map(|(i, (dec, inc))| {
            if dec >= time && inc >= time {
                Some(i)
            } else {
                None
            }
        })
        .collect()
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
        assert_eq!(good_days_to_rob_bank(&[5, 3, 3, 3, 5, 6, 2], 2), [2, 3]);
        assert_eq!(good_days_to_rob_bank(&[1, 1, 1, 1, 1], 0), [0, 1, 2, 3, 4]);
        assert!(good_days_to_rob_bank(&[1, 2, 3, 4, 5, 6], 2).is_empty());
    }

    #[test]
    fn test() {}
}
