mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
    words.sort_unstable_by_key(|s| s.len());
    let mut trie = Trie::default();
    let mut res = 0;
    for s in &words {
        res += trie.insert(s.bytes().rev());
    }
    res
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    len: i32,
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = u8>) -> i32 {
        let mut curr = self;
        let mut res = 0;
        let mut len = 1;
        for b in it {
            len += 1;
            let i = usize::from(b - b'a');
            let node = curr.nodes[i].get_or_insert_default();
            res -= node.len;
            node.len = 0;
            curr = node;
        }
        curr.len = len;
        res + len
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
    fn basics() {}

    #[test]
    fn test() {}
}
