mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_unsorted_subarray(nums: &[i32]) -> i32 {
    let mut stack = vec![];
    let (mut left_i, mut max_popped): (Option<usize>, _) = (None, None);
    for (idx, &num) in nums.iter().enumerate() {
        if stack.is_empty() {
            stack.push((idx, num));
        }
        while stack.last().is_some_and(|&(_, v)| v > num) {
            let (i, n) = stack.pop().unwrap();
            if let Some(v) = left_i.as_mut() {
                *v = (*v).min(i);
            } else {
                left_i = Some(i)
            }
            max_popped = max_popped.max(Some(n));
        }
        stack.push((idx, num));
    }
    if left_i.is_none() {
        return 0;
    }
    let right_i = stack
        .iter()
        .find_map(|&(i, v)| {
            if v >= max_popped.unwrap() {
                Some(i)
            } else {
                None
            }
        })
        .unwrap_or(nums.len());
    (right_i - left_i.unwrap()) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_unsorted_subarray(&[2, 6, 4, 8, 10, 9, 15]), 5);
        debug_assert_eq!(find_unsorted_subarray(&[1, 2, 3, 4]), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_unsorted_subarray(&[1, 3, 2, 2, 2]), 4);
        debug_assert_eq!(find_unsorted_subarray(&[1, 5, 3, 2, 4]), 4);
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
