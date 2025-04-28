mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
    let mut res = vec![];
    for q in words_query.iter() {
        let mut trie = Trie::default();
        trie.insert(q.bytes().rev());
        let mut maxi = 0;
        let mut max_suf = 0;
        let mut len = usize::MAX;
        for (i, s) in words_container.iter().enumerate() {
            let temp = trie.find(s.bytes().rev());
            if temp > max_suf {
                max_suf = temp;
                len = s.len();
                maxi = i;
            } else if temp == max_suf && s.len() < len {
                len = s.len();
                maxi = i;
            }
        }
        res.push(maxi as i32);
    }
    res
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    len: usize,
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = u8>) {
        let mut curr = self;
        for (i, b) in it.enumerate() {
            let idx = usize::from(b - b'a');
            curr = curr.nodes[idx].get_or_insert_default();
            curr.len = 1 + i;
        }
    }

    fn find(&self, it: impl Iterator<Item = u8>) -> usize {
        let mut curr = self;
        let mut res = curr.len;
        for b in it {
            let idx = usize::from(b - b'a');
            let Some(ref v) = curr.nodes[idx] else {
                break;
            };
            curr = v;
            res = curr.len;
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
