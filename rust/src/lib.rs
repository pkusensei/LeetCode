mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn palindrome_pairs(words: &[&str]) -> Vec<Vec<i32>> {
    let mut trie = Trie::default();
    for (i, s) in words.iter().enumerate() {
        trie.insert(s.as_bytes(), i);
    }
    let mut res = vec![];
    for (i, s) in words.iter().enumerate() {
        trie.search(s.as_bytes(), i, &mut res);
    }
    res
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    idx: Option<usize>,
    indices: Vec<usize>,
}

impl Trie {
    fn insert(&mut self, s: &[u8], idx: usize) {
        let mut curr = self;
        for (right, &b) in s.iter().enumerate().rev() {
            let node = curr.nodes[usize::from(b - b'a')].get_or_insert_default();
            if is_palindrome(&s[..=right]) {
                curr.indices.push(idx); // From this point to the start, it is palin
            }
            curr = node;
        }
        // records this node is a word's start(since loop backwards)
        curr.idx = Some(idx);
    }

    fn search(&self, s: &[u8], idx: usize, res: &mut Vec<Vec<i32>>) {
        let mut curr = self;
        for (si, &b) in s.iter().enumerate() {
            if let Some(curr_idx) = curr.idx
                && curr_idx != idx
                && is_palindrome(&s[si..])
            {
                // Found a word (curr.idx is Some) and the rest of queried s is palindrome
                res.push(vec![idx as i32, curr_idx as _]);
            }
            let Some(v) = curr.nodes[usize::from(b - b'a')].as_ref() else {
                return;
            };
            curr = v;
        }
        if let Some(curr_idx) = curr.idx
            && curr_idx != idx
        {
            // Check this node
            res.push(vec![idx as i32, curr_idx as _]);
        }
        for &i in &curr.indices {
            if i != idx {
                res.push(vec![idx as i32, i as i32]);
            }
        }
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
        sort_eq!(
            palindrome_pairs(&["abcd", "dcba", "lls", "s", "sssll"]),
            [[0, 1], [1, 0], [3, 2], [2, 4]]
        );
        sort_eq!(palindrome_pairs(&["bat", "tab", "cat"]), [[0, 1], [1, 0]]);
        sort_eq!(palindrome_pairs(&["a", ""]), [[0, 1], [1, 0]]);
    }

    #[test]
    fn test() {
        sort_eq!(
            palindrome_pairs(&["a", "abc", "aba", ""]),
            [[0, 3], [3, 0], [2, 3], [3, 2]]
        );
    }
}
