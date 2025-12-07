mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sort_by_reflection(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable_by(|a, b| {
        let [sa, sb] = [a, b].map(|v| format!("{:b}", v).chars().rev().collect::<String>());
        let [ra, rb] = [sa, sb].map(|s| u64::from_str_radix(&s, 2).unwrap_or_default());
        ra.cmp(&rb).then(a.cmp(b))
    });
    nums
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
    fn basics() {}

    #[test]
    fn test() {}
}
