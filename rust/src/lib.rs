mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_good(statements: &[&[i32]]) -> i32 {
    let n = statements.len();
    let mut res = 0;
    for mask in 0..1 << n {
        check(statements, mask, &mut res);
    }
    res as _
}

fn check(states: &[&[i32]], mask: i32, res: &mut u32) {
    let [mut good, mut bad] = [0, 0];
    for (idx, row) in states.iter().enumerate() {
        if (mask >> idx) & 1 == 1 {
            for (i, &v) in row.iter().enumerate() {
                match v {
                    0 => bad |= 1 << i,
                    1 => good |= 1 << i,
                    _ => (),
                }
            }
        }
        if good & bad > 0 {
            return;
        }
    }
    if bad & mask == 0 && good & mask == good {
        *res = (*res).max(mask.count_ones())
    }
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
        assert_eq!(maximum_good(&[&[2, 1, 2], &[1, 2, 2], &[2, 0, 2]]), 2);
        assert_eq!(maximum_good(&[&[2, 0], &[0, 2]]), 1);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_good(&[&[2, 2], &[1, 2]]), 2);
        assert_eq!(
            maximum_good(&[
                &[2, 0, 2, 2, 0],
                &[2, 2, 2, 1, 2],
                &[2, 2, 2, 1, 2],
                &[1, 2, 0, 2, 2],
                &[1, 0, 2, 1, 2]
            ]),
            2
        );
        assert_eq!(
            maximum_good(&[&[2, 2, 2, 2], &[1, 2, 1, 0], &[0, 2, 2, 2], &[0, 0, 0, 2]]),
            1
        );
    }
}
