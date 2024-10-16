mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn daily_temperatures(temperatures: &[i32]) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = vec![];
    let mut res = vec![0; temperatures.len()];
    for (idx, &num) in temperatures.iter().enumerate() {
        while stack.last().is_some_and(|&(_, n)| n < num) {
            let Some((i, _)) = stack.pop() else {
                break;
            };
            res[i] = (idx - i) as i32;
        }
        stack.push((idx, num));
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            daily_temperatures(&[73, 74, 75, 71, 69, 72, 76, 73]),
            [1, 1, 4, 2, 1, 1, 0, 0]
        );
        debug_assert_eq!(daily_temperatures(&[30, 40, 50, 60]), [1, 1, 1, 0]);
        debug_assert_eq!(daily_temperatures(&[30, 60, 90]), [1, 1, 0]);
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
