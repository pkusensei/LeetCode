mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut count = roads.iter().fold(vec![0; n as usize], |mut acc, r| {
        acc[r[0] as usize] += 1;
        acc[r[1] as usize] += 1;
        acc
    });
    count.sort_unstable_by(|a, b| b.cmp(a));
    count
        .iter()
        .zip((1..=n).rev())
        .map(|(c, v)| i64::from(v) * c)
        .sum()
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
