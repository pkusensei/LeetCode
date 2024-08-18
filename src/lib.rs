mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Default)]
struct Trie {
    nodes: [Option<Node>; 26],
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: &str) {
        let bytes = word.as_bytes();
        let idx = (bytes[0] - b'a') as usize;
        if self.nodes[idx].is_none() {
            self.nodes[idx] = Some(Node::new(bytes));
        } else {
            let Some(n) = self.nodes[idx].as_mut() else {
                return;
            };
            n.insert(bytes);
        }
    }

    fn search(&self, word: &str) -> bool {
        let bytes = word.as_bytes();
        let idx = (bytes[0] - b'a') as usize;
        if let Some(n) = self.nodes[idx].as_ref() {
            n.search(bytes)
        } else {
            false
        }
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let bytes = prefix.as_bytes();
        let idx = (bytes[0] - b'a') as usize;
        if let Some(n) = self.nodes[idx].as_ref() {
            n.starts_with(bytes)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Node {
    val: u8,
    is_end: bool,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    fn new(bytes: &[u8]) -> Self {
        let mut node = Self {
            val: bytes[0],
            is_end: false,
            children: Default::default(),
        };
        if bytes.len() > 1 {
            node.insert(bytes)
        } else {
            node.is_end = true;
        }
        node
    }

    fn insert(&mut self, bytes: &[u8]) {
        debug_assert_eq!(self.val, bytes[0]);
        if bytes.len() > 1 {
            let child = bytes[1];
            if let Some(ref mut n) = self.children[(child - b'a') as usize] {
                n.insert(&bytes[1..])
            } else {
                let node = Self::new(&bytes[1..]);
                self.children[(child - b'a') as usize] = Some(Box::new(node))
            }
        } else {
            self.is_end = true;
        }
    }

    fn search(&self, bytes: &[u8]) -> bool {
        debug_assert_eq!(self.val, bytes[0]);
        if bytes.len() == 1 {
            self.is_end
        } else {
            let child = (bytes[1] - b'a') as usize;
            if let Some(ref n) = self.children[child] {
                n.search(&bytes[1..])
            } else {
                false
            }
        }
    }

    fn starts_with(&self, bytes: &[u8]) -> bool {
        debug_assert_eq!(self.val, bytes[0]);
        if bytes.len() == 1 {
            true
        } else {
            let child = (bytes[1] - b'a') as usize;
            if let Some(ref n) = self.children[child] {
                n.starts_with(&bytes[1..])
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
        let mut trie = Trie::new();
        trie.insert("apple");
        debug_assert!(trie.search("apple")); // return True
        debug_assert!(!trie.search("app")); // return False
        debug_assert!(trie.starts_with("app")); // return True
        trie.insert("app");
        debug_assert!(trie.search("app")); // return True
    }

    #[test]
    fn test() {}
}
