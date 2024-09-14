mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subarray(nums: &[i32]) -> i32 {
    let (mut curr, mut curr_c) = (0, 0);
    let (mut prev, mut prev_c) = (0, 0);
    for &num in nums.iter() {
        match curr.cmp(&num) {
            std::cmp::Ordering::Less => {
                curr = num;
                curr_c = 1
            }
            std::cmp::Ordering::Equal => curr_c += 1,
            std::cmp::Ordering::Greater => {
                if curr > prev {
                    prev = curr;
                    prev_c = curr_c;
                } else if curr == prev {
                    prev_c = prev_c.max(curr_c);
                }
                curr = 0;
                curr_c = 0;
            }
        }
    }
    match curr.cmp(&prev) {
        std::cmp::Ordering::Less => prev_c,
        std::cmp::Ordering::Equal => curr_c.max(prev_c),
        std::cmp::Ordering::Greater => curr_c,
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_subarray(&[1, 2, 3, 3, 2, 2]), 2);
        debug_assert_eq!(longest_subarray(&[1, 2, 3, 4]), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            longest_subarray(&[
                311155, 311155, 311155, 311155, 311155, 311155, 311155, 311155, 201191, 311155
            ]),
            8
        );
    }

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
