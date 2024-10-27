mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn push_dominoes(dominoes: &str) -> String {
    let n = dominoes.len();
    let mut forces = Vec::with_capacity(n);
    let mut curr = 0;
    for b in dominoes.bytes() {
        curr = match b {
            b'R' => n as i32,
            b'L' => 0,
            // influence of domino decreases per distance
            _ => (curr - 1).max(0),
        };
        forces.push(curr);
    }
    curr = 0;
    for (i, b) in dominoes.bytes().enumerate().rev() {
        curr = match b {
            b'R' => 0,
            b'L' => n as i32,
            _ => (curr - 1).max(0),
        };
        forces[i] -= curr;
    }
    forces
        .into_iter()
        .map(|f| match f.cmp(&0) {
            std::cmp::Ordering::Less => 'L',
            std::cmp::Ordering::Equal => '.',
            std::cmp::Ordering::Greater => 'R',
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(push_dominoes("RR.L"), "RR.L");
        debug_assert_eq!(push_dominoes(".L.R...LR..L.."), "LL.RR.LLRRLL..");
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
