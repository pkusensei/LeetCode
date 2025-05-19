mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_valid_strings(words: &[&str], target: &str) -> i32 {
    let mut trie = Trie::default();
    for s in words.iter() {
        trie.insert(s.bytes());
    }
    let (s, n) = (target.as_bytes(), target.len());
    let mut dp = vec![i32::MAX / 2; 1 + n];
    dp[0] = 0;
    for left in 0..n {
        if dp[left] >= i32::MAX / 2 {
            continue;
        }
        let mut node = &trie;
        for right in left..n {
            let Some(v) = node.get(s[right]) else {
                break;
            };
            node = v;
            dp[1 + right] = dp[1 + right].min(1 + dp[left]);
        }
    }
    if dp[n] >= i32::MAX / 2 { -1 } else { dp[n] }
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = u8>) {
        let mut curr = self;
        for b in it {
            let idx = usize::from(b - b'a');
            curr = curr.nodes[idx].get_or_insert_default();
        }
    }

    fn get(&self, b: u8) -> Option<&Trie> {
        let idx = usize::from(b - b'a');
        self.nodes[idx].as_deref()
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
        assert_eq!(min_valid_strings(&["abc", "aaaaa", "bcdef"], "aabcdabc"), 3);
        assert_eq!(min_valid_strings(&["abababab", "ab"], "ababaababa"), 2);
        assert_eq!(min_valid_strings(&["abcdef"], "xyz"), -1);
    }

    #[test]
    fn test() {}
}
