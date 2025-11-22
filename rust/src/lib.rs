mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn lex_smallest_negated_perm(n: i32, target: i64) -> Vec<i32> {
    let n = i64::from(n);
    let sum = (1 + n) * n / 2;
    let t_abs = target.abs();
    let delta = sum - t_abs;
    if delta < 0 || delta & 1 == 1 {
        return vec![];
    }
    let mut res = Vec::with_capacity(n as usize);
    let mut delta = sum - target;
    for num in (1..=n).rev() {
        if delta > 0 && 2 * num <= delta {
            res.push(-num);
            delta -= 2 * num;
        } else {
            res.push(num);
        }
    }
    res.sort_unstable();
    res.into_iter().map(|v| v as i32).collect()
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
        assert_eq!(lex_smallest_negated_perm(3, 0), [-3, 1, 2]);
        assert_eq!(lex_smallest_negated_perm(3, -2), [-3, -1, 2])
    }

    #[test]
    fn test() {
        assert_eq!(lex_smallest_negated_perm(4, -4), [-4, -3, 1, 2]);
    }
}
