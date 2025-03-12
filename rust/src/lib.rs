mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distribute_cookies(cookies: &[i32], k: i32) -> i32 {
    let k = k as usize;
    let mut buckets = vec![0; k];
    backtrack(cookies, &mut buckets)
}

fn backtrack(cookies: &[i32], buckets: &mut [i32]) -> i32 {
    match cookies {
        [] => *buckets.iter().max().unwrap(),
        [head, tail @ ..] => {
            let k = buckets.len();
            let mut res = i32::MAX;
            for i in 0..k {
                buckets[i] += head;
                res = res.min(backtrack(tail, buckets));
                buckets[i] -= head;
            }
            res
        }
    }
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
        assert_eq!(distribute_cookies(&[8, 15, 10, 20, 8], 2), 31);
        assert_eq!(distribute_cookies(&[6, 1, 3, 2, 2, 4, 1, 2], 3), 7);
    }

    #[test]
    fn test() {}
}
