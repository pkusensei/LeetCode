mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_distribute(nums: &[i32], quantity: &mut [i32]) -> bool {
    let mut counts: Vec<_> = nums
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_values()
        .collect();
    quantity.sort_unstable_by_key(|&v| std::cmp::Reverse(v)); // WHAT??!!
    backtrack(&mut counts, quantity)
}

fn backtrack(counts: &mut [i32], quantity: &[i32]) -> bool {
    match quantity {
        [] => true,
        [head, tail @ ..] => {
            let n = counts.len();
            for i in 0..n {
                if counts[i] >= *head {
                    counts[i] -= head;
                    if backtrack(counts, tail) {
                        return true;
                    }
                    counts[i] += head;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert!(!can_distribute(&[1, 2, 3, 4], &mut [2]));
        assert!(can_distribute(&[1, 2, 3, 3], &mut [2]));
        assert!(can_distribute(&[1, 1, 2, 2], &mut [2, 2]));
    }

    #[test]
    fn test() {
        assert!(can_distribute(
            &[1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            &mut [3, 3, 3, 4]
        ));
        assert!(!can_distribute(
            &[
                1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
                13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23,
                24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 32, 32, 33, 33, 34,
                34, 35, 35, 36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42, 43, 43, 44, 44,
                45, 45, 46, 46, 47, 47, 48, 48, 49, 49, 50, 50
            ],
            &mut [2, 2, 2, 2, 2, 2, 2, 2, 2, 3]
        ));
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
