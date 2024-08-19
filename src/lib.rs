mod helper;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn find_words(board: &[&[char]], words: &[&str]) -> Vec<String> {
    let (row, col) = get_dimensions(board);
    if row * col == 0 {
        return vec![];
    }

    let mut res = vec![];
    let mut curr = String::new();
    let mut seen = HashSet::new();
    let dict: WordDictionary = words.iter().collect();
    for (y, row) in board.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            dfs(board, &dict, &mut res, &mut curr, (x, y), ch, &mut seen);
        }
    }
    res.sort_unstable();
    res
}

fn dfs(
    board: &[&[char]],
    dict: &WordDictionary,
    res: &mut Vec<String>,
    curr: &mut String,
    coord: Coord,
    ch: char,
    seen: &mut HashSet<Coord>,
) {
    if !seen.insert(coord) {
        return;
    }
    curr.push(ch);

    if !dict.starts_with(curr) || curr.len() > 10 {
        seen.remove(&coord);
        curr.pop();
        return;
    }
    if let Some(n) = dict.search(curr) {
        res.push(curr.clone());
        n.is_end.set(false);
    }

    for (neighbor, &ch) in neighbors(coord)
        .filter_map(|(x, y)| board.get(y).and_then(|v| v.get(x)).map(|ch| ((x, y), ch)))
    {
        dfs(board, dict, res, curr, neighbor, ch, seen);
    }
    seen.remove(&coord);
    curr.pop();
}

#[derive(Debug, Clone, Default)]
struct WordDictionary {
    nodes: [Option<Trie>; 26],
}

impl<T: AsRef<str>> FromIterator<T> for WordDictionary {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut dict = Self::new();
        for s in iter {
            dict.add_word(s.as_ref());
        }
        dict
    }
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: &str) {
        let b = word.as_bytes()[0];
        let node = self.nodes[(b - b'a') as usize].get_or_insert(Trie::new());
        node.insert(&word[1..])
    }

    fn search(&self, word: &str) -> Option<&Trie> {
        let b = word.as_bytes()[0];
        // if b == b'.' {
        //     self.nodes
        //         .iter()
        //         .filter_map(|n| n.as_ref())
        //         .find(|n| n.search(&word[1..]))
        //         .is_some()
        // } else
        if let Some(ref node) = self.nodes[(b - b'a') as usize] {
            node.search(&word[1..])
        } else {
            None
        }
    }

    fn starts_with(&self, word: &str) -> bool {
        let b = word.as_bytes()[0];
        if let Some(ref node) = self.nodes[(b - b'a') as usize] {
            node.starts_with(&word[1..])
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: std::cell::Cell<bool>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for b in word.bytes() {
            node = node.children[(b - b'a') as usize].get_or_insert(Box::new(Self::new()));
        }
        node.is_end.set(true);
    }

    fn search(&self, word: &str) -> Option<&Self> {
        if let Some(n) = self.find(word) {
            if n.is_end.get() {
                return Some(n);
            }
        }
        None
        // if word.is_empty() {
        //     self.is_end
        // } else {
        //     let b = word.as_bytes()[0];
        //     if b == b'.' {
        //         self.children
        //             .iter()
        //             .filter_map(|n| n.as_ref())
        //             .find(|n| n.search(&word[1..]))
        //             .is_some()
        //     } else
        //     if let Some(ref n) = self.children[(b - b'a') as usize] {
        //         n.search(&word[1..])
        //     } else {
        //         false
        //     }
        // }
    }

    fn starts_with(&self, word: &str) -> bool {
        self.find(word).is_some()
    }

    fn find(&self, word: &str) -> Option<&Self> {
        if word.is_empty() {
            Some(self)
        } else {
            let b = word.as_bytes()[0];
            if let Some(ref n) = self.children[(b - b'a') as usize] {
                n.find(&word[1..])
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_words(
                &[
                    &['o', 'a', 'a', 'n'],
                    &['e', 't', 'a', 'e'],
                    &['i', 'h', 'k', 'r'],
                    &['i', 'f', 'l', 'v']
                ],
                &["oath", "pea", "eat", "rain"]
            ),
            ["eat", "oath"]
        );
        debug_assert!(find_words(&[&['a', 'b'], &['c', 'd']], &["abcb"]).is_empty())
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            find_words(
                &[
                    &['o', 'a', 'b', 'n'],
                    &['o', 't', 'a', 'e'],
                    &['a', 'h', 'k', 'r'],
                    &['a', 'f', 'l', 'v']
                ],
                &["oa", "oaa"]
            ),
            ["oa", "oaa"]
        );
        debug_assert_eq!(
            find_words(
                &[
                    &['o', 'a', 'a', 'n'],
                    &['e', 't', 'a', 'e'],
                    &['i', 'h', 'k', 'r'],
                    &['i', 'f', 'l', 'v']
                ],
                &["oath", "pea", "eat", "rain", "hklf", "hf"]
            ),
            ["eat", "hf", "hklf", "oath"]
        );
    }
}
