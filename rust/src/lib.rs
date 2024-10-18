mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_largest_special(s: &str) -> String {
    let bytes = s.as_bytes();
    String::from_utf8(build(&bytes)).unwrap()
}

fn build(bytes: &[u8]) -> Vec<u8> {
    if bytes.len() <= 2 {
        return bytes.to_vec(); // at worst "()" <= "10"
    }
    let mut open = 0;
    let mut start = 0;
    let mut pairs = vec![];
    for (idx, &b) in bytes.iter().enumerate() {
        match b {
            b'1' => open += 1,
            b'0' => {
                open -= 1;
                if open == 0 {
                    pairs.push((start, idx));
                    start = idx + 1;
                }
            }
            _ => unreachable!(),
        }
    }
    if pairs.len() == 1 {
        // "(()())" strip outer bytes => "()()"
        let mut v = build(&bytes[1..bytes.len() - 1]);
        v.reserve(2);
        v.insert(0, b'1');
        v.push(b'0');
        v
    } else {
        // "(())()" split => "(())" and "()"
        // sort to put bigger bits front
        let mut res: Vec<_> = pairs
            .into_iter()
            .map(|(start, end)| build(&bytes[start..=end]))
            .collect();
        res.sort_unstable_by(|a, b| b.cmp(&a));
        res.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(make_largest_special("11011000"), "11100100");
        debug_assert_eq!(make_largest_special("10"), "10");
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
