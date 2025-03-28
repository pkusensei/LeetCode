mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut curr = 0;
    for b in word.bytes() {
        let d = i64::from(b - b'0');
        curr = (10 * curr + d) % i64::from(m);
        res.push(i32::from(curr == 0));
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
