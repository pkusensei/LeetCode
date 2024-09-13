mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn xor_queries(arr: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(arr.len());
    prefix.push(arr[0]);
    for (i, &num) in arr.iter().enumerate().skip(1) {
        prefix.push(prefix[i - 1] ^ num);
    }
    queries
        .iter()
        .map(|&[left, right]| {
            if left > 0 {
                prefix[right as usize] ^ prefix[left as usize - 1]
            } else {
                prefix[right as usize]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            xor_queries(&[1, 3, 4, 8], &[[0, 1], [1, 2], [0, 3], [3, 3]]),
            [2, 7, 14, 8]
        );
        debug_assert_eq!(
            xor_queries(&[4, 8, 2, 10], &[[2, 3], [1, 3], [0, 0], [0, 3]]),
            [8, 0, 4, 4]
        );
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
