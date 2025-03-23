mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_jump(stones: &[i32]) -> i32 {
    let n = stones.len();
    let mut idx = 0;
    let mut res = 0;
    let mut prev = 0;
    while idx < n {
        res = res.max(stones[idx] - prev);
        if idx == n - 1 {
            break;
        }
        prev = stones[idx];
        idx = (idx + 2).min(n - 1);
    }
    idx = 1;
    prev = 0;
    while idx < n {
        res = res.max(stones[idx] - prev);
        if idx == n - 1 {
            break;
        }
        prev = stones[idx];
        idx = (idx + 2).min(n - 1);
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
        assert_eq!(max_jump(&[0, 2, 5, 6, 7]), 5);
        assert_eq!(max_jump(&[0, 3, 9]), 9);
    }

    #[test]
    fn test() {}
}
