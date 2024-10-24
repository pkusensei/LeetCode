mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_lines(widths: &[i32], s: &str) -> [i32; 2] {
    const MAX: i32 = 100;
    let (mut line, mut count) = (0, 0);
    for num in s.bytes().map(|b| widths[usize::from(b - b'a')]) {
        if count + num <= MAX {
            count += num;
        } else {
            line += 1;
            count = num
        }
    }
    [1 + line, count]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            number_of_lines(
                &[
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz"
            ),
            [3, 60]
        );
        debug_assert_eq!(
            number_of_lines(
                &[
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa"
            ),
            [2, 4]
        );
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
