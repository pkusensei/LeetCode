mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_removal(beans: &mut [i32]) -> i64 {
    let n = beans.len();
    beans.sort_unstable();
    let prefix = beans.iter().fold(Vec::with_capacity(n), |mut acc, &num| {
        acc.push(i64::from(num) + acc.last().unwrap_or(&0));
        acc
    });
    let mut res = i64::MAX;
    for (idx, &num) in beans.iter().enumerate() {
        let curr = prefix[n - 1] - i64::from(num) * (n - idx) as i64;
        res = res.min(curr);
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
        assert_eq!(minimum_removal(&mut [4, 1, 6, 5]), 4);
        assert_eq!(minimum_removal(&mut [2, 10, 3, 2]), 7);
    }

    #[test]
    fn test() {}
}
