mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn robot_with_string(s: &str) -> String {
    let mut freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut t = vec![]; // stack
    let mut res = vec![];
    let mut upper = 0;
    for b in s.bytes().map(|b| usize::from(b - b'a')) {
        t.push(b);
        freq[b] -= 1;
        while freq.get(upper).is_some_and(|&v| v == 0) {
            upper += 1; // Find big char still left in s
        }
        while t.last().is_some_and(|&v| v <= upper) {
            let byte = t.pop().unwrap() as u8 + b'a';
            res.push(byte); // Append all smaller chars
        }
    }
    String::from_utf8(res).unwrap()
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
        assert_eq!(robot_with_string("bdda"), "addb");
    }

    #[test]
    fn test() {}
}
