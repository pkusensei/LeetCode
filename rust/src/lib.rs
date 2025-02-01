mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_gain(s: &str, x: i32, y: i32) -> i32 {
    let mut st1 = vec![];
    let &[mut left, mut right] = if x > y { b"ab" } else { b"ba" };
    let mut res = 0;
    for b in s.bytes() {
        if b == right && st1.last().is_some_and(|&v| v == left) {
            st1.pop();
            res += x.max(y);
        } else {
            st1.push(b);
        }
    }
    std::mem::swap(&mut left, &mut right);
    let mut st2 = vec![];
    for b in st1 {
        if b == right && st2.last().is_some_and(|&v| v == left) {
            st2.pop();
            res += x.min(y)
        } else {
            st2.push(b);
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(maximum_gain("cdbcbbaaabab", 4, 5), 19);
        assert_eq!(maximum_gain("aabbaaxybbaabb", 5, 4), 20);
    }

    #[test]
    fn test() {}
}
