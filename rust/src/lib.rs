mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_prefix_scores(words: &[&str]) -> Vec<i32> {
    let mut indices: Vec<_> = (0..words.len()).collect();
    indices.sort_unstable_by(|&a, &b| words[a].cmp(words[b]));
    let lengths = common_prefix_lengths(words, &indices);
    calc_scores(words, &indices, &lengths)
    // with_trie(words)
}

// map words to indices
// sort indices by the words they point to
// get common prefix between "adjacent" words
// for (i, j) in indices => ([i], [j]) in words
// the further removed, the less common [i] and [j] are
// accumulate common([i], [j]) before it becomes 0 eventually
fn common_prefix_lengths(words: &[&str], indices: &[usize]) -> Vec<i32> {
    let n = words.len();
    let mut res = Vec::with_capacity(n);
    res.push(0);
    for w in indices.windows(2) {
        let (a, b) = (words[w[0]], words[w[1]]);
        res.push(a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count() as i32);
    }
    res
}

fn calc_scores(words: &[&str], indices: &[usize], lengths: &[i32]) -> Vec<i32> {
    let n = words.len();
    let mut res = vec![0; n];
    for (idx, &word_idx) in indices.iter().enumerate() {
        let word_length = words[word_idx].len() as i32;
        res[word_idx] += word_length;
        let mut j = idx + 1;
        let mut common_length = word_length;
        while j < n {
            common_length = common_length.min(lengths[j]);
            if common_length == 0 {
                break;
            }
            res[word_idx] += common_length;
            res[indices[j]] += common_length;
            j += 1
        }
    }
    res
}

fn with_trie(words: &[&str]) -> Vec<i32> {
    let mut trie = Trie::new();
    for s in words.iter() {
        trie.add(s.as_bytes());
    }
    words.iter().map(|s| trie.count(s.as_bytes())).collect()
}

#[derive(Debug, Clone)]
struct Trie {
    data: [Option<Box<Trie>>; 26],
    count: i32,
}

impl Trie {
    const fn new() -> Self {
        Self {
            data: [const { None }; 26],
            count: 0,
        }
    }

    fn add(&mut self, s: &[u8]) {
        let [b, tail @ ..] = s else {
            return;
        };
        let idx = usize::from(b - b'a');
        if let Some(node) = self.data.get_mut(idx).and_then(|n| n.as_mut()) {
            node.count += 1;
            node.add(tail);
        } else {
            let mut node = Self::new();
            node.count += 1;
            node.add(tail);
            self.data[idx] = Some(Box::new(node));
        }
    }

    fn count(&self, s: &[u8]) -> i32 {
        match s {
            [] => self.count,
            [b, tail @ ..] => {
                let idx = usize::from(b - b'a');
                if let Some(node) = self.data.get(idx).and_then(|n| n.as_ref()) {
                    self.count + node.count(tail)
                } else {
                    self.count
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(sum_prefix_scores(&["abc", "ab", "bc", "b"]), [5, 4, 3, 2]);
        debug_assert_eq!(sum_prefix_scores(&["abcd"]), [4]);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
