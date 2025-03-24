mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_partition(s: &str, k: i32) -> i32 {
    let n = s.len();
    let len = 1 + k.ilog10() as usize;
    let mut prev = 0;
    let mut res = 0;
    let mut curr = len.min(n);
    while curr <= n {
        while curr > prev && s[prev..curr].parse::<i32>().is_ok_and(|v| v > k) {
            curr -= 1;
        }
        if curr == prev {
            return -1;
        }
        res += 1;
        prev = curr;
        curr = (prev + len).min(n);
        if prev == n {
            break;
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
        assert_eq!(minimum_partition("165462", 60), 4);
        assert_eq!(minimum_partition("238182", 5), -1);
    }

    #[test]
    fn test() {}
}
