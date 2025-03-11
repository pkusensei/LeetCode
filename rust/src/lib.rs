mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_substrings(s: String) -> i32 {
    let mut prev = [-1; 3];
    let mut res = 0;
    for (idx, b) in (0..).zip(s.bytes()) {
        prev[usize::from(b - b'a')] = idx;
        res += 1 + prev.iter().min().unwrap_or(&-1);
        // a b c a b
        //     ^   ^
        // this means, each substr starting in [a b c] is valid
        // i.e += 1+ index_of(c)
        // Before that, until every char is included, the min is -1
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
