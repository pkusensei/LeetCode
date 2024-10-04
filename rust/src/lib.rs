mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn divide_players(skill: &mut [i32]) -> i64 {
    let n = skill.len();
    let sum: i32 = skill.iter().sum();
    if (sum * 2) % n as i32 > 0 {
        return -1;
    }
    let ave = sum * 2 / n as i32;
    skill.sort_unstable();
    let it = skill.iter().take(n / 2).zip(skill.iter().rev().take(n / 2));
    if !it.clone().all(|(a, b)| a + b == ave) {
        return -1;
    }
    it.map(|(&a, &b)| i64::from(a) * i64::from(b)).sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(divide_players(&mut [3, 2, 5, 1, 3, 4]), 22);
        debug_assert_eq!(divide_players(&mut [3, 4]), 12);
        debug_assert_eq!(divide_players(&mut [1, 1, 2, 3]), -1);
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
