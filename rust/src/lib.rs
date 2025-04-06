mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn survived_robots_healths(positions: &[i32], healths: &[i32], directions: &str) -> Vec<i32> {
    use itertools::{Itertools, izip};
    // (id, pos, health, dir)
    let robs = izip!(positions.iter(), healths.iter(), directions.bytes())
        .enumerate()
        .map(|(i, (&p, &h, b))| (i, p, h, b))
        .sorted_unstable_by_key(|t| t.1)
        .collect_vec();
    // (id, health, dir)
    let mut stack: Vec<(usize, i32, u8)> = vec![];
    'outer: for rob in robs {
        let mut curr = (rob.0, rob.2, rob.3);
        while stack.last().is_some_and(|&v| v.2 == b'R' && curr.2 == b'L') {
            let v = stack.pop().unwrap();
            match v.1.cmp(&curr.1) {
                std::cmp::Ordering::Less => curr.1 -= 1,
                std::cmp::Ordering::Equal => continue 'outer,
                std::cmp::Ordering::Greater => {
                    curr = v;
                    curr.1 -= 1
                }
            }
        }
        stack.push(curr);
    }
    stack.sort_unstable_by_key(|v| v.0);
    stack.into_iter().map(|v| v.1).collect()
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
        assert_eq!(
            survived_robots_healths(&[5, 4, 3, 2, 1], &[2, 17, 9, 15, 10], "RRRRR"),
            [2, 17, 9, 15, 10]
        );
        assert_eq!(
            survived_robots_healths(&[3, 5, 2, 6], &[10, 10, 15, 12], "RLRL"),
            [14]
        );
        assert_eq!(
            survived_robots_healths(&[1, 2, 5, 6], &[10, 10, 11, 11], "RLRL"),
            []
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            survived_robots_healths(&[11, 44, 16], &[1, 20, 17], "RLR"),
            [18]
        );
    }
}
