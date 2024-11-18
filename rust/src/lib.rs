mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn common_chars(words: &[&str]) -> Vec<String> {
    let mut chs = [u16::MAX; 26];
    for s in words.iter() {
        for (a, b) in chs.iter_mut().zip(count(s)) {
            if *a > 0 {
                (*a) = (*a).min(b);
            }
        }
    }
    chs.into_iter()
        .enumerate()
        .flat_map(|(i, v)| {
            std::iter::repeat(char::from(i as u8 + b'a').to_string()).take(v as usize)
        })
        .collect()
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
        debug_assert_eq!(common_chars(&["bella", "label", "roller"]), ["e", "l", "l"]);
        debug_assert_eq!(common_chars(&["cool", "lock", "cook"]), ["c", "o"]);
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
