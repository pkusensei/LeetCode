mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_subsequence_count(text: &str, pattern: &str) -> i64 {
    let [c1, c2] = [0, 1].map(|i| pattern.as_bytes()[i]);
    let mut curr = [0, 0];
    let mut res = 0;
    for b in text.bytes() {
        if b == c2 {
            res += curr[0];
            curr[1] += 1;
        }
        // Put b==c1 second to handle case when c1==c2
        if b == c1 {
            curr[0] += 1;
        }
    }
    // max => the added char at either end
    res + curr[0].max(curr[1])
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
        assert_eq!(maximum_subsequence_count("abdcdbc", "ac"), 4);
        assert_eq!(maximum_subsequence_count("aabb", "ab"), 6);
    }

    #[test]
    fn test() {
        assert_eq!(
            maximum_subsequence_count("iekbksdsmuuzwxbpmcngsfkjvpzuknqguzvzik", "mp"),
            5
        );
    }
}
