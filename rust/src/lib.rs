mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_groups_for_valid_assignment(balls: &[i32]) -> i32 {
    use itertools::Itertools;
    let freq = balls.iter().copied().counts();
    if freq.len() == 1 {
        return 1;
    }
    let min = *freq.values().min().unwrap_or(&1);
    let mut res = freq.values().sum::<usize>() as i32;
    'outer: for f in (1..=min).rev() {
        let f1 = 1 + f;
        let mut curr = 0;
        for &count in freq.values() {
            let gr = count / f1;
            let last = count % f1;
            if last > 0 && last + gr < f {
                continue 'outer; // borrow from at most gr groups
            }
            curr += gr + usize::from(last > 0);
        }
        res = res.min(curr as i32);
        break;
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
        assert_eq!(min_groups_for_valid_assignment(&[3, 2, 3, 2, 3]), 2);
        assert_eq!(min_groups_for_valid_assignment(&[10, 10, 10, 3, 1, 1]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(min_groups_for_valid_assignment(&[2, 3, 2, 2, 2]), 3);
        assert_eq!(min_groups_for_valid_assignment(&[1, 1, 3, 1, 1, 3]), 3);
    }
}
