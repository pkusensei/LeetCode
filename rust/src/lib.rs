mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn next_greater_element(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let mut stack = vec![];
    let mut map = std::collections::HashMap::new();
    for &num in nums2.iter().rev() {
        if stack.is_empty() {
            stack.push(num);
            continue;
        }
        while stack.last().is_some_and(|&v| v < num) {
            stack.pop();
        }
        if let Some(last) = stack.last().copied() {
            map.entry(num).or_insert(last);
        }
        stack.push(num);
    }
    nums1
        .iter()
        .map(|n| map.get(n).copied().unwrap_or(-1))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(next_greater_element(&[4, 1, 2], &[1, 3, 4, 2]), [-1, 3, -1]);
        debug_assert_eq!(next_greater_element(&[2, 4], &[1, 2, 3, 4]), [3, -1]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            next_greater_element(&[1, 3, 5, 2, 4], &[6, 5, 4, 3, 2, 1, 7]),
            &[7, 7, 7, 7, 7]
        );
    }

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
