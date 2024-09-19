#[derive(Debug, Clone)]
pub struct Trie<const N: usize> {
    data: [Option<Box<Trie<N>>>; N],
    is_end: bool,
}

impl<const N: usize> Default for Trie<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl<const N: usize> Trie<N> {
    pub const fn new() -> Self {
        Self {
            data: [const { None }; N],
            is_end: false,
        }
    }

    pub fn insert<T, I, F>(&mut self, input: I, index_of: F)
    where
        I: IntoIterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        self.insert_impl(input.into_iter(), index_of)
    }

    fn insert_impl<T, I, F>(&mut self, mut it: I, mut index_of: F)
    where
        I: Iterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        if let Some(v) = it.next() {
            let idx = index_of(v);
            if let Some(n) = self.data.get_mut(idx).and_then(|opt| opt.as_mut()) {
                n.insert(it, index_of);
            } else {
                let mut node = Box::new(Self::new());
                node.insert(it, index_of);
                self.data[idx] = Some(node);
            }
        } else {
            self.is_end = true;
        }
    }

    pub fn check<T, I, F>(&self, input: I, index_of: F) -> bool
    where
        I: IntoIterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        self.check_impl(input.into_iter(), index_of)
    }

    fn check_impl<T, I, F>(&self, mut it: I, mut index_of: F) -> bool
    where
        I: Iterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        if let Some(v) = it.next() {
            let idx = index_of(v);
            if let Some(node) = self.data.get(idx).and_then(|n| n.as_ref()) {
                return node.check(it, index_of);
            } else {
                return false;
            }
        }
        self.is_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "trie not changed"]
    fn basics() {
        let mut trie = Trie::<26>::new();
        let func = |&b| usize::from(b - b'a');
        trie.insert(b"abcde", func);
        debug_assert!(trie.check(b"abcde", func));
        debug_assert!(!trie.check(b"abc", func));
        trie.insert(b"abc", func);
        debug_assert!(trie.check(b"abc", func));
        debug_assert!(!trie.check(b"abcd", func));
        debug_assert!(!trie.check(b"abcdef", func));
        debug_assert!(trie.check(b"abcde", func));

        debug_assert!(!trie.check(b"abdef", func));
        trie.insert(b"abdef", func);
        debug_assert!(trie.check(b"abdef", func));
        debug_assert!(trie.check(b"abcde", func));

        debug_assert!(!trie.check(b"x", func));
        trie.insert(b"xyz", func);
        debug_assert!(trie.check(b"xyz", func));
        debug_assert!(!trie.check(b"x", func));
    }
}
