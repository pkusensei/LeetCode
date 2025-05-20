mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_substrings(word: &str, k: i32) -> i32 {
        let (s, n) = (word.as_bytes(), word.len());
        let mut res = 0;
        for left in 0..=(n - 5 - k as usize) {
            let mut freq = [0; 5];
            let mut cons = 0;
            for right in left..n {
                if let Some(i) = find(s[right]) {
                    freq[i] += 1;
                } else {
                    cons += 1;
                }
                res += i32::from(cons == k && freq.iter().all(|&v| v >= 1));
                if cons > k {
                    break;
                }
            }
        }
        res
}

fn find(b: u8) -> Option<usize> {
    b"aeiou".iter().position(|&v| v == b)
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
    fn test() {
        assert_eq!(count_of_substrings("iqeaouqi", 2), 3)
    }
}
