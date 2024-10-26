mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn mask_pii(s: &str) -> String {
    if let Some((name, dom)) = s.split_once('@') {
        let name = name.as_bytes();
        let (a, b) = (name[0], name.last().copied().unwrap());
        let mut v = vec![a.to_ascii_lowercase()];
        v.extend_from_slice(b"*****");
        v.push(b.to_ascii_lowercase());
        let r = String::from_utf8(v).unwrap();
        format!("{}@{}", r, dom.to_ascii_lowercase())
    } else {
        let digits: Vec<_> = s.bytes().filter(|b| b.is_ascii_digit()).collect();
        let mut code = match digits.len() {
            11 => b"+*-***-***-".to_vec(),
            12 => b"+**-***-***-".to_vec(),
            13 => b"+***-***-***-".to_vec(),
            _ => b"***-***-".to_vec(),
        };
        code.extend_from_slice(&digits[digits.len() - 4..]);
        String::from_utf8(code).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(mask_pii("LeetCode@LeetCode.com"), "l*****e@leetcode.com");
        debug_assert_eq!(mask_pii("AB@qq.com"), "a*****b@qq.com");
        debug_assert_eq!(mask_pii("1(234)567-890"), "***-***-7890");
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
