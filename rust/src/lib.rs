mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    nums.chunks(3)
        .map(|ch| {
            if ch[2] - ch[0] <= k {
                Some(ch.to_vec())
            } else {
                None
            }
        })
        .collect::<Option<Vec<Vec<i32>>>>()
        .unwrap_or_default()
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
    fn basics() {}

    #[test]
    fn test() {}
}
