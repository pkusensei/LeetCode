mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn get_hint(secret: &str, guess: &str) -> String {
    let (mut a, mut b) = (0, 0);
    let (mut m, mut g) = (vec![0; 10], vec![0; 10]);
    for (byte1, byte2) in secret.bytes().zip(guess.bytes()) {
        if byte1 == byte2 {
            a += 1;
        } else {
            m[usize::from(byte1 - b'0')] += 1;
            g[usize::from(byte2 - b'0')] += 1;
        }
    }
    for (c1, c2) in m.into_iter().zip(g) {
        b += c1.min(c2);
    }
    format!("{a}A{b}B")
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(get_hint("1807", "7810"), "1A3B");
        debug_assert_eq!(get_hint("1123", "0111"), "1A1B");
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
