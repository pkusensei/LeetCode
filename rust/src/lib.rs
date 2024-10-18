mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// https://en.wikipedia.org/wiki/De_Bruijn_sequence#Algorithm
pub fn crack_safe(n: i32, k: i32) -> String {
    let alphabet: Vec<u8> = (0..k).map(|v| v as u8).collect();
    let (n, k) = (n as usize, k as usize);
    if k == 1 {
        return std::iter::repeat('0').take(n * k).collect();
    }
    let mut a = vec![0; k * n];
    let mut seq = vec![];
    db(1, 1, n, k, &mut a, &mut seq);
    seq.extend_from_within(..n - 1);
    seq.into_iter()
        .map(|i| char::from(b'0' + alphabet[i]))
        .collect()
}

fn db(t: usize, p: usize, n: usize, k: usize, a: &mut [usize], seq: &mut Vec<usize>) {
    if t > n {
        if n % p == 0 {
            seq.extend_from_slice(&a[1..1 + p]);
        }
    } else {
        a[t] = a[t - p];
        db(1 + t, p, n, k, a, seq);
        for i in 1 + a[t - p]..k {
            a[t] = i;
            db(1 + t, t, n, k, a, seq);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(crack_safe(1, 2), "01");
        debug_assert_eq!(crack_safe(2, 2), "00110");
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
