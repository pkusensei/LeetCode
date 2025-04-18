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
        let mut curr = self;
        for item in input {
            let idx = (curr.index_of)(item);
            curr = curr.data[idx].get_or_insert(Box::new(Trie::new(curr.index_of.clone())))
        }
        curr.is_end = true;
    }

    pub fn check<T, I>(&self, input: I) -> bool
    where
        I: IntoIterator<Item = T>,
        F: Fn(T) -> usize,
    {
        let mut curr = self;
        for item in input {
            let idx = (curr.index_of)(item);
            let Some(ref v) = curr.data[idx] else {
                return false;
            };
            curr = v;
        }
        curr.is_end
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
