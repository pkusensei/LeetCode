mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_two_no_overlap(nums: &[i32], first_len: i32, second_len: i32) -> i32 {
    let n = nums.len();
    let [len1, len2] = [first_len, second_len].map(|v| v as usize);
    let prefix = nums
        .iter()
        .fold((Vec::with_capacity(n), 0), |(mut acc, sum), &num| {
            let sum = sum + num;
            acc.push(sum);
            (acc, sum)
        })
        .0;
    let mut res = 0;
    let mut seen = vec![-1; n];
    for i1 in 0..=n - len1 {
        let s1 = if i1 == 0 {
            prefix[i1 + len1 - 1]
        } else {
            prefix[i1 + len1 - 1] - prefix[i1 - 1]
        };
        let mut s2 = 0;
        if i1 >= len2 {
            for i2 in 0..=i1 - len2 {
                let temp = {
                    if seen[i2] >= 0 {
                        seen[i2]
                    } else {
                        seen[i2] = if i2 == 0 {
                            prefix[i2 + len2 - 1]
                        } else {
                            prefix[i2 + len2 - 1] - prefix[i2 - 1]
                        };
                        seen[i2]
                    }
                };
                s2 = s2.max(temp);
            }
        }
        if i1 + len1 + len2 <= n {
            for i2 in i1 + len1..=n - len2 {
                let temp = if seen[i2] >= 0 {
                    seen[i2]
                } else {
                    seen[i2] = prefix[i2 + len2 - 1] - prefix[i2 - 1];
                    seen[i2]
                };
                s2 = s2.max(temp);
            }
        }
        res = res.max(s1 + s2);
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
            max_sum_two_no_overlap(&[0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
            20
        );
        debug_assert_eq!(
            max_sum_two_no_overlap(&[3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
            29
        );
        debug_assert_eq!(
            max_sum_two_no_overlap(&[2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
            31
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
