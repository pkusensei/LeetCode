mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct Encrypter {
    key_vals: [Option<String>; 26],
    val_keys: Vec<Vec<Vec<usize>>>,
    trie: Trie,
}

impl Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mut key_vals = [const { None }; 26];
        let mut val_keys = vec![vec![vec![]; 26]; 26];
        for (k, v) in keys.iter().zip(values.iter()) {
            key_vals[usize::from(*k as u8 - b'a')] = Some(v.clone());
            let [a, b] = v.as_bytes()[..] else {
                unreachable!()
            };
            let [a, b] = [a, b].map(|v| usize::from(v - b'a'));
            val_keys[a][b].push(usize::from(*k as u8 - b'a'));
        }
        let mut trie = Trie::default();
        for s in dictionary.iter() {
            let it = s.bytes().map(|b| usize::from(b - b'a'));
            trie.insert(it);
        }
        Self {
            key_vals,
            val_keys,
            trie,
        }
    }

    fn encrypt(&self, word1: String) -> String {
        let mut res = String::new();
        for i in word1.bytes().map(|b| usize::from(b - b'a')) {
            let Some(ref v) = self.key_vals[i] else {
                return String::new();
            };
            res.push_str(v);
        }
        res
    }

    fn decrypt(&self, word2: String) -> i32 {
        self.dfs(&self.trie, word2.as_bytes())
    }

    fn dfs(&self, trie: &Trie, word2: &[u8]) -> i32 {
        if word2.is_empty() {
            return i32::from(trie.is_end);
        }
        let [a, b] = [0, 1].map(|i| usize::from(word2[i] - b'a'));
        let mut res = 0;
        for &next in self.val_keys[a][b].iter() {
            if let Some(ref node) = trie.nodes[next] {
                res += self.dfs(node, &word2[2..]);
            };
        }
        res
    }
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn insert<I>(&mut self, it: I)
    where
        I: Iterator<Item = usize>,
    {
        let mut curr = self;
        for v in it {
            let node = curr.nodes[v].get_or_insert(Box::new(Trie::default()));
            curr = node;
        }
        curr.is_end = true;
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
    fn basics() {
        let mut en = Encrypter::new(
            vec!['a', 'b', 'c', 'd'],
            vec!["ei".into(), "zf".into(), "ei".into(), "am".into()],
            vec![
                "abcd".into(),
                "acbd".into(),
                "adbc".into(),
                "badc".into(),
                "dacb".into(),
                "cadb".into(),
                "cbda".into(),
                "abad".into(),
            ],
        );
        assert_eq!(en.encrypt("abcd".into()), "eizfeiam");
        // 'a' maps to "ei", 'b' maps to "zf", 'c' maps to "ei", and 'd' maps to "am".
        assert_eq!(en.decrypt("eizfeiam".into()), 2);
        // "ei" can map to 'a' or 'c', "zf" maps to 'b', and "am" maps to 'd'.
        // Thus, the possible strings after decryption are "abad", "cbad", "abcd", and "cbcd".
        // 2 of those strings, "abad" and "abcd", appear in dictionary, so the answer is 2.
    }

    #[test]
    fn test() {}
}
