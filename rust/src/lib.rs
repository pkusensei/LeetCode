mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_initial_strength(monsters: &[i32], boosts: &[[i32; 3]]) -> i64 {
    use itertools::izip;
    let n = monsters.len();
    let mut diff = vec![0; n];
    for b in boosts {
        let [left, right, v] = b[..] else {
            unreachable!()
        };
        let v = i64::from(v);
        diff[left as usize] += v;
        if (1 + right as usize) < n {
            diff[1 + right as usize] -= v;
        }
    }
    let mut left = 0;
    let mut right: i64 = monsters.iter().map(|&v| i64::from(v)).sum();
    'out: while left < right {
        let mid = left + (right - left) / 2;
        let mut curr = mid;
        let mut prefix = 0;
        for (&mon, &d) in izip!(monsters.iter(), diff.iter()) {
            let mon = i64::from(mon);
            prefix += d;
            if mon > curr + prefix {
                left = 1 + mid;
                continue 'out;
            }
            curr = (curr - mon).max(0);
        }
        right = mid;
    }
    left
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
    fn basics() {
        assert_eq!(
            min_initial_strength(&[5, 10, 15], &[[1, 2, 10], [1, 2, 5]]),
            5
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            min_initial_strength(
                &[738260286],
                &[
                    [0, 0, 779345931],
                    [0, 0, 322031493],
                    [0, 0, 512569541],
                    [0, 0, 621512189],
                    [0, 0, 322982288]
                ]
            ),
            0
        );
    }
}
