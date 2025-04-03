mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_extra_char(s: &str, dictionary: &[&str]) -> i32 {
    let mut trie = Trie::default();
    let mut max_len = 0; // of word in dict
    for w in dictionary.iter() {
        trie.add(w.bytes());
        max_len = max_len.max(w.len());
    }
    let n = s.len();
    let mut dp = vec![n; 1 + n];
    dp[0] = 0;
    for end in 1..=n {
        for left in end.saturating_sub(max_len)..end {
            let len = end - left;
            let curr = if trie.find(s[left..end].bytes(), len) {
                dp[left]
            } else {
                dp[left] + len
            };
            dp[end] = dp[end].min(curr);
        }
    }
    dp[n] as i32
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    len: usize,
}

impl Trie {
    fn add(&mut self, it: impl Iterator<Item = u8>) {
        let mut curr = self;
        let mut len = 0;
        for b in it {
            let idx = usize::from(b - b'a');
            curr = curr.nodes[idx].get_or_insert(Box::new(Trie::default()));
            len += 1;
        }
        curr.len = len;
    }

    fn find(&self, it: impl Iterator<Item = u8>, len: usize) -> bool {
        let mut curr = self;
        for b in it {
            let idx = usize::from(b - b'a');
            let Some(ref v) = curr.nodes[idx] else {
                break;
            };
            curr = v;
        }
        curr.len == len
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
        assert_eq!(
            min_extra_char("leetscode", &["leet", "code", "leetcode"]),
            1
        );
        assert_eq!(min_extra_char("sayhelloworld", &["hello", "world"]), 3);
    }

    #[test]
    fn test() {}
}
