mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(mut nums: Vec<i32>, queries: &[i32]) -> Vec<i64> {
    let n = nums.len();
    nums.sort_unstable();
    let prefix = nums.iter().fold(vec![], |mut acc, &num| {
        acc.push(i64::from(num) + acc.last().unwrap_or(&0));
        acc
    });
    let mut res = vec![];
    for &q in queries.iter() {
        let i = nums.partition_point(|&v| v <= q);
        let q = i64::from(q);
        if i == 0 {
            res.push(prefix[n - 1] - q * n as i64);
        } else {
            let left = i as i64 * q - prefix[i - 1];
            let right = prefix[n - 1] - prefix[i - 1] - (n - i) as i64 * q;
            res.push(left + right);
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
        assert_eq!(min_operations(vec![3, 1, 6, 8], &[1, 5]), [14, 10]);
        assert_eq!(min_operations(vec![2, 9, 6, 3], &[10]), [20]);
    }

    #[test]
    fn test() {}
}
