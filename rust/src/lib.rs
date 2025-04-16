mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distribute_candies(n: i32, limit: i32) -> i64 {
    let mut res = 0;
    for a in 0..=n.min(limit) {
        // b => [0, (n-a).min(limit)]
        // c => [0, (n-a-b).min(limit)]
        let max_c = (n - a).min(limit); // b==0
        let min_c = (n - a - limit).max(0); // b==limit
        res += i64::from(max_c - min_c + 1).max(0);
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
        assert_eq!(distribute_candies(5, 2), 3);
        assert_eq!(distribute_candies(3, 3), 10);
    }

    #[test]
    fn test() {}
}
