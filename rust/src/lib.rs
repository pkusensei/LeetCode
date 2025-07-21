mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for b in word.bytes() {
            curr = curr.nodes[usize::from(b - b'a')].get_or_insert_default();
        }
        curr.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut curr = self;
        for b in word.bytes() {
            let Some(node) = curr.nodes[usize::from(b - b'a')].as_ref() else {
                return false;
            };
            curr = node;
        }
        curr.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for b in prefix.bytes() {
            let Some(node) = curr.nodes[usize::from(b - b'a')].as_ref() else {
                return false;
            };
            curr = node;
        }
        true
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
