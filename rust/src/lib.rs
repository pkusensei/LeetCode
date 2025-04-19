mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
    coins.sort_unstable();
    let mut curr = 1;
    let mut idx = 0;
    let mut res = 0;
    while curr <= target {
        // Suppose v is reachable,
        // Adding v makes all [v..=2v] reachable
        if coins.get(idx).is_some_and(|&v| v <= curr) {
            curr += coins[idx];
            idx += 1
        } else {
            res += 1;
            curr *= 2;
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
        assert_eq!(minimum_added_coins(vec![1, 4, 10], 19), 2);
        assert_eq!(minimum_added_coins(vec![1, 4, 10, 5, 7, 19], 19), 1);
        assert_eq!(minimum_added_coins(vec![1, 1, 1], 20), 3);
    }

    #[test]
    fn test() {}
}
