mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
    let mut trie = Trie::default();
    for d in dictionary.iter() {
        trie.insert(d.bytes());
    }
    queries
        .into_iter()
        .filter(|s| trie.find(s.as_bytes(), 2))
        .collect()
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    end: bool,
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = u8>) {
        let mut curr = self;
        for b in it {
            let i = usize::from(b - b'a');
            curr = curr.nodes[i].get_or_insert_default();
        }
        curr.end = true;
    }

    fn find(&self, it: &[u8], count: i32) -> bool {
        if it.is_empty() {
            return self.end && count >= 0;
        }
        if count < 0 {
            return false;
        }
        let bi = usize::from(it[0] - b'a');
        let mut res = false;
        for (i, node) in self.nodes.iter().enumerate() {
            if let Some(v) = node {
                if i == bi {
                    res |= v.find(&it[1..], count)
                } else {
                    res |= v.find(&it[1..], count - 1)
                }
            }
        }
        res
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
