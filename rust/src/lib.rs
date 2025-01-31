mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves(nums: &[i32], k: i32) -> i32 {
    let ones: Vec<_> = nums
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == 1 { Some(i) } else { None })
        .collect();
    let prefix = ones.iter().fold(vec![1], |mut acc, &val| {
        acc.push(val + acc.last().unwrap_or(&1));
        acc
    });
    let (n, k) = (ones.len(), k as usize);
    (0..=n - k)
        .map(|i| {
            (prefix[i + k] - prefix[i + (k + 1) / 2])
                - (prefix[i + k / 2] - prefix[i])
                - k / 2 * (1 + k) / 2
        })
        .min()
        .unwrap_or(0) as _
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(min_moves(&[1, 0, 0, 1, 0, 1], 2), 1);
        assert_eq!(min_moves(&[1, 0, 0, 0, 0, 0, 1, 1], 3), 5);
        assert_eq!(min_moves(&[1, 1, 0, 1], 2), 0);
    }

    #[test]
    fn test() {}
}
