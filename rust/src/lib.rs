mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
    let mut trie = Trie::new();
    for (i, s) in words_container.iter().enumerate() {
        trie.insert(s.bytes().rev(), i, s.len());
    }
    let mut res = vec![];
    for q in words_query.iter() {
        res.push(trie.find(q.bytes().rev()) as i32);
    }
    res
}

struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    shortest: usize,
    str_idx: usize,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: [const { None }; 26],
            shortest: 10_001,
            str_idx: 10_001,
        }
    }

    fn insert(&mut self, it: impl Iterator<Item = u8>, idx: usize, len: usize) {
        let mut curr = self;
        if curr.shortest > len {
            curr.shortest = len;
            curr.str_idx = idx;
        }
        for b in it {
            let ci = usize::from(b - b'a');
            curr = curr.nodes[ci].get_or_insert(Box::new(Self::new()));
            if curr.shortest > len {
                curr.shortest = len;
                curr.str_idx = idx;
            }
        }
    }

    fn find(&self, it: impl Iterator<Item = u8>) -> usize {
        let mut curr = self;
        let mut res = self.str_idx;
        for b in it {
            let idx = usize::from(b - b'a');
            let Some(ref v) = curr.nodes[idx] else {
                break;
            };
            curr = v;
            res = curr.str_idx;
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
