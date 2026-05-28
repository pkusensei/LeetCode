mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn string_indices(words_container: &[&str], words_query: &[&str]) -> Vec<i32> {
    let mut t = Trie::default();
    for (i, s) in words_container.iter().enumerate() {
        t.insert(s.bytes().rev(), s.len(), i);
    }
    words_query
        .iter()
        .map(|s| t.find(s.bytes().rev()) as i32)
        .collect()
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    min_len: usize,
    idx: usize,
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = u8>, len: usize, idx: usize) {
        let mut curr = self;
        for b in it {
            if curr.min_len == 0 || curr.min_len > len {
                curr.min_len = len;
                curr.idx = idx;
            }
            curr = curr.nodes[usize::from(b - b'a')].get_or_insert_default();
        }
        if curr.min_len == 0 || curr.min_len > len {
            curr.min_len = len;
            curr.idx = idx;
        }
    }

    fn find(&self, it: impl Iterator<Item = u8>) -> usize {
        let mut curr = self;
        for b in it {
            if let Some(ref v) = curr.nodes[usize::from(b - b'a')] {
                curr = v
            } else {
                break;
            }
        }
        curr.idx
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
        assert_eq!(
            string_indices(&["abcd", "bcd", "xbcd"], &["cd", "bcd", "xyz"]),
            [1, 1, 1]
        );
    }

    #[test]
    fn test() {}
}
