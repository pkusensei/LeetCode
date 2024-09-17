mod helper;

use std::{collections::HashMap, iter};

#[allow(unused_imports)]
use helper::*;

pub fn original_digits(s: &str) -> String {
    let mut letters = s.bytes().fold(HashMap::new(), |mut acc, b| {
        *acc.entry(b).or_insert(0) += 1;
        acc
    });
    let mut res = vec![];
    res.extend(remove_word(&mut letters, b'z', b"zero", b'0'));
    res.extend(remove_word(&mut letters, b'w', b"two", b'2'));
    res.extend(remove_word(&mut letters, b'u', b"four", b'4'));
    res.extend(remove_word(&mut letters, b'x', b"six", b'6'));
    res.extend(remove_word(&mut letters, b'g', b"eight", b'8'));
    res.extend(remove_word(&mut letters, b'o', b"one", b'1'));
    res.extend(remove_word(&mut letters, b'h', b"three", b'3'));
    res.extend(remove_word(&mut letters, b'f', b"five", b'5'));
    res.extend(remove_word(&mut letters, b'v', b"seven", b'7'));
    res.extend(remove_word(&mut letters, b'i', b"nine", b'9'));

    res.sort_unstable();
    res.into_iter().map(char::from).collect()
}

fn remove_word(letters: &mut HashMap<u8, i32>, unique: u8, seq: &[u8], digit: u8) -> Vec<u8> {
    let mut res = vec![];
    if let Some(&count) = letters.get(&unique) {
        res.extend(iter::repeat(digit).take(count as usize));
        letters.remove(&b'z');
        for &b in seq.iter() {
            if let Some(v) = letters.get_mut(&b) {
                *v -= count;
            }
        }
    };
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(original_digits("owoztneoer"), "012");
        debug_assert_eq!(original_digits("fviefuro"), "45");
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
