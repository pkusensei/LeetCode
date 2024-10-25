#[derive(Debug, Clone)]
pub struct Trie<F, const N: usize> {
    data: [Option<Box<Trie<F, N>>>; N],
    is_end: bool,
    index_of: F,
}

#[allow(dead_code)]
impl<F, const N: usize> Trie<F, N> {
    pub const fn new(index_of: F) -> Self {
        Self {
            data: [const { None }; N],
            is_end: false,
            index_of,
        }
    }

    pub fn insert<T, I>(&mut self, input: I)
    where
        I: IntoIterator<Item = T>,
        F: Fn(T) -> usize + Clone,
    {
        self.insert_impl(input.into_iter())
    }

    fn insert_impl<T, I>(&mut self, mut it: I)
    where
        I: Iterator<Item = T>,
        F: Fn(T) -> usize + Clone,
    {
        if let Some(v) = it.next() {
            let idx = (self.index_of)(v);
            if let Some(n) = self.data.get_mut(idx).and_then(|opt| opt.as_mut()) {
                n.insert(it);
            } else {
                let mut node = Box::new(Self::new(self.index_of.clone()));
                node.insert(it);
                self.data[idx] = Some(node);
            }
        } else {
            self.is_end = true;
        }
    }

    pub fn check<T, I>(&self, input: I) -> bool
    where
        I: IntoIterator<Item = T>,
        F: Fn(T) -> usize,
    {
        self.check_impl(input.into_iter())
    }

    fn check_impl<T, I>(&self, mut it: I) -> bool
    where
        I: Iterator<Item = T>,
        F: Fn(T) -> usize,
    {
        if let Some(v) = it.next() {
            let idx = (self.index_of)(v);
            if let Some(node) = self.data.get(idx).and_then(|n| n.as_ref()) {
                return node.check(it);
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
        let mut trie: Trie<_, 26> = Trie::new(|b: &u8| usize::from(b - b'a'));
        trie.insert(b"abcde");
        debug_assert!(trie.check(b"abcde",));
        debug_assert!(!trie.check(b"abc",));
        trie.insert(b"abc");
        debug_assert!(trie.check(b"abc",));
        debug_assert!(!trie.check(b"abcd",));
        debug_assert!(!trie.check(b"abcdef",));
        debug_assert!(trie.check(b"abcde",));

        debug_assert!(!trie.check(b"abdef",));
        trie.insert(b"abdef");
        debug_assert!(trie.check(b"abdef",));
        debug_assert!(trie.check(b"abcde",));

        debug_assert!(!trie.check(b"x",));
        trie.insert(b"xyz");
        debug_assert!(trie.check(b"xyz",));
        debug_assert!(!trie.check(b"x",));
    }
}
