mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_tasks(tasks: &[i32], shifts: &[i32]) -> Vec<i32> {
    let n = tasks.len();
    let prefix = tasks.iter().fold(Vec::with_capacity(n), |mut acc, &v| {
        acc.push(i64::from(v) + acc.last().unwrap_or(&0));
        acc
    });
    let mut res = Vec::with_capacity(shifts.len());
    let mut total = 0;
    for &shift in shifts.iter() {
        total += i64::from(shift);
        let i = prefix.partition_point(|&v| v <= total);
        if i == n {
            res.push(0);
            total = 0;
        } else {
            res.push((n - i) as i32);
        }
    }
    res
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
        assert_eq!(count_tasks(&[2, 3, 4], &[20, 4, 5]), [0, 2, 0]);
        assert_eq!(count_tasks(&[4, 2], &[3, 6, 1]), [2, 0, 2])
    }

    #[test]
    fn test() {
        assert_eq!(
            count_tasks(&[1, 1, 3, 3, 8], &[2, 9, 5, 3, 9]),
            [3, 1, 0, 3, 1]
        );
    }
}
