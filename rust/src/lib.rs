mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn remove_duplicate_letters(s: &str) -> String {
    let mut count = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut seen = [false; 26]; // track what's in stack
    let mut stack = vec![];
    for b in s.bytes() {
        let idx = usize::from(b - b'a');
        count[idx] -= 1;
        if seen[idx] {
            continue;
        }
        while stack
            .last()
            .is_some_and(|&v| b < v && count[usize::from(v - b'a')] > 0)
        {
            // pop "bigger" chars unless this is their final occurrence
            let Some(v) = stack.pop() else {
                break;
            };
            seen[usize::from(v - b'a')] = false
        }
        stack.push(b);
        seen[idx] = true;
    }
    stack.into_iter().map(char::from).collect()
    // solve(s.as_bytes().to_vec())
    //     .into_iter()
    //     .map(char::from)
    //     .collect()
}

fn solve(mut s: Vec<u8>) -> Vec<u8> {
    if s.is_empty() {
        return s;
    }
    let mut count = s.iter().fold([0; 26], |mut acc, &b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let (mut idx, mut byte) = (0, s[0]);
    for (i, &b) in s.iter().enumerate() {
        if b < byte {
            idx = i;
            byte = b;
        }
        let c = usize::from(b - b'a');
        count[c] -= 1;
        if count[c] == 0 {
            break;
        }
    }
    let mut rest = s.split_off(idx + 1);
    rest.retain(|&b| b != byte);
    let mut res = solve(rest);
    res.insert(0, byte);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(remove_duplicate_letters("bcabc"), "abc");
        debug_assert_eq!(remove_duplicate_letters("cbacdcbc"), "acdb");
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
