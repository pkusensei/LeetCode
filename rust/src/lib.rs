mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_bit(n: i32, k: i32) -> char {
    let mut curr = vec![0u8];
    for _ in 0..n {
        let mut inv = curr.clone();
        for v in inv.iter_mut() {
            *v = 1 - *v;
        }
        curr = curr
            .into_iter()
            .chain(std::iter::once(1))
            .chain(inv.into_iter().rev())
            .collect();
    }
    (b'0' + curr[k as usize - 1]).into()
}

fn recurse(n: i32, k: i32) -> char {
    if n == 1 {
        return '0';
    }
    let len = 1 << n;
    match k.cmp(&(len / 2)) {
        std::cmp::Ordering::Less => recurse(n - 1, k),
        std::cmp::Ordering::Equal => '1',
        std::cmp::Ordering::Greater => {
            if '0' == find_kth_bit(n - 1, len - k) {
                '1'
            } else {
                '0'
            }
        }
    }
}

fn divide_and_conquer(n: i32, mut k: i32) -> char {
    let mut invert = 0;
    let mut len = 1 << n - 1;
    while k > 1 {
        if k == len / 2 + 1 {
            return if invert & 1 == 0 { '0' } else { '1' };
        }
        if k > len / 2 + 1 {
            k = len + 1 - k;
            invert += 1;
        }
        len /= 2;
    }
    if invert & 1 == 0 {
        '0'
    } else {
        '1'
    }
}

fn bit_magic(_n: i32, k: i32) -> char {
    let pos_in_section = k & -k;
    let in_inverted_sec = (((k / pos_in_section) >> 1) & 1) == 1;
    let original_bit_is_one = (k & 1) == 0;
    match (in_inverted_sec, original_bit_is_one) {
        (true, true) => '0',
        (true, false) => '1',
        (false, true) => '1',
        (false, false) => '0',
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(bit_magic(3, 1), '0');
        debug_assert_eq!(bit_magic(4, 11), '1');
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
