mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_special_chars(word: &str) -> i32 {
    let [mut low, mut up] = [[None; 26]; 2];
    for (i, b) in word.bytes().enumerate() {
        if b.is_ascii_lowercase() {
            low[usize::from(b - b'a')] = Some(i)
        } else if b.is_ascii_uppercase() {
            up[usize::from(b - b'A')].get_or_insert(i);
        }
    }
    low.iter()
        .zip(up)
        .filter(|(a, b)| a.zip(*b).is_some_and(|(a, b)| a < b))
        .count() as i32
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(number_of_special_chars("aaAbcBC"), 3);
    }

    #[test]
    fn test() {}
}
