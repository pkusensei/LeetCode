mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_anagrams(s: &str, p: &str) -> Vec<i32> {
    let (sn, pn) = (s.len(), p.len());
    if sn < pn {
        return vec![];
    }
    let target = count(p);
    let mut curr = count(&s[..pn]);
    let s = s.as_bytes();
    let mut res = vec![];
    if target == curr {
        res.push(0);
    }
    for idx in 1..=sn - pn {
        curr[usize::from(s[idx + pn - 1] - b'a')] += 1;
        curr[usize::from(s[idx - 1] - b'a')] -= 1;
        if target == curr {
            res.push(idx as i32);
        }
    }
    res
}

fn count(s: &str) -> [u16; 26] {
    s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(find_anagrams("cbaebabacd", "abc"), [0, 6]);
        sort_eq(find_anagrams("abab", "ab"), [0, 1, 2]);
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
