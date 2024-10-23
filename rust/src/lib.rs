mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swap(nums1: &[i32], nums2: &[i32]) -> i32 {
    // dfs(&nums1[1..], &nums2[1..], 0, nums1[0], nums2[0]).unwrap_or(-1)

    let (mut swap, mut skip) = (1, 0); // [skip, swap]
    for (w1, w2) in nums1.windows(2).zip(nums2.windows(2)) {
        if w1[0] >= w2[1] || w2[0] >= w1[1] {
            // For window
            // ...[3, 5]...
            // ...[2, 3]...
            // If [2]<->[3] is swapped, [5]<->[3] has to as well
            // keep skip the same
            swap += 1 // mark this [5]<->[3] swap
        } else if w1[0] >= w1[1] || w2[0] >= w2[1] {
            // Have to swap
            // skip = swap to count possible previous swap
            std::mem::swap(&mut swap, &mut skip);
            swap += 1;
        } else {
            skip = skip.min(swap);
            swap = 1 + skip;
        }
    }
    swap.min(skip)
}

fn dfs(nums1: &[i32], nums2: &[i32], count: i32, prev1: i32, prev2: i32) -> Option<i32> {
    match (nums1, nums2) {
        ([], []) => Some(count),
        ([h1, t1 @ ..], [h2, t2 @ ..]) => {
            let (h1, h2) = (*h1, *h2);
            if prev1 >= h1 || prev2 >= h2 {
                // must switch
                if prev1 < h2 && prev2 < h1 {
                    dfs(t1, t2, 1 + count, h2, h1)
                } else {
                    None // not possible; terminate this path
                }
            } else if prev1 < h2 && prev2 < h1 {
                let switch = dfs(t1, t2, 1 + count, h2, h1);
                let skip = dfs(t1, t2, count, h1, h2);
                match (switch, skip) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (None, None) => None,
                    _ => switch.or(skip),
                }
            } else {
                dfs(t1, t2, count, h1, h2)
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_swap(&[1, 3, 5, 4], &[1, 2, 3, 7]), 1);
        debug_assert_eq!(min_swap(&[0, 3, 5, 8, 9], &[2, 1, 4, 6, 9]), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(min_swap(&[0, 4, 4, 5, 9], &[0, 1, 6, 8, 10]), 1);
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
