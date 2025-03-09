mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
    colors.extend_from_within(..k as usize - 1);
    let mut len = 1;
    let mut res = 0;
    for (idx, &color) in colors.iter().enumerate().skip(1) {
        if color != colors[idx - 1] {
            len += 1;
            res += i32::from(len >= k)
        } else {
            len = 1;
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
        assert_eq!(number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3), 3);
        assert_eq!(
            number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
            2
        );
        assert_eq!(number_of_alternating_groups(vec![1, 1, 0, 1], 4), 0);
    }

    #[test]
    fn test() {}
}
