mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
    let mut res = n - 1;
    let mut prev = 0;
    let mut time = 0;
    for log in logs.iter() {
        let [id, end] = log[..] else { unreachable!() };
        let curr = end - prev;
        prev = end;
        if curr > time {
            time = curr;
            res = id
        } else if curr == time {
            res = res.min(id)
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
    fn basics() {}

    #[test]
    fn test() {}
}
