mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct WordDictionary {
    nodes: [Option<Trie>; 26],
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

    fn search(&self, word: &str) -> bool {
        let b = word.as_bytes()[0];
        if b == b'.' {
            self.nodes
                .iter()
                .filter_map(|n| n.as_ref())
                .find(|n| n.search(&word[1..]))
                .is_some()
        } else if let Some(ref node) = self.nodes[(b - b'a') as usize] {
            node.search(&word[1..])
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
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
        node.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        if word.is_empty() {
            self.is_end
        } else {
            let b = word.as_bytes()[0];
            if b == b'.' {
                self.children
                    .iter()
                    .filter_map(|n| n.as_ref())
                    .find(|n| n.search(&word[1..]))
                    .is_some()
            } else if let Some(ref n) = self.children[(b - b'a') as usize] {
                n.search(&word[1..])
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut dict = WordDictionary::new();
        dict.add_word("bad");
        dict.add_word("dad");
        dict.add_word("mad");
        debug_assert!(!dict.search("pad")); // return False
        debug_assert!(dict.search("bad")); // return True
        debug_assert!(dict.search(".ad")); // return True
        debug_assert!(dict.search("b..")); // return True
    }

    #[test]
    fn test() {}
}
