mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
    let mut n = 1;
    loop {
        if memory1 >= memory2 && memory1 >= n {
            memory1 -= n;
        } else if memory2 >= n {
            memory2 -= n
        } else {
            return vec![n, memory1, memory2];
        }
        n += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
