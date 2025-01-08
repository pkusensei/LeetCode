mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_multiple_of_three(mut digits: Vec<i32>) -> String {
    digits.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
    let sum: i32 = digits.iter().sum();
    match sum % 3 {
        1 => {
            if let Some(i) = digits.iter().rposition(|&v| v % 3 == 1) {
                digits.remove(i);
            } else {
                let temp: Vec<_> = digits
                    .iter()
                    .enumerate()
                    .rev()
                    .filter_map(|(i, &v)| if v % 3 == 2 { Some(i) } else { None })
                    .take(2)
                    .collect();
                for i in temp {
                    digits.remove(i);
                }
            }
        }
        2 => {
            if let Some(i) = digits.iter().rposition(|&v| v % 3 == 2) {
                digits.remove(i);
            } else {
                let temp: Vec<_> = digits
                    .iter()
                    .enumerate()
                    .rev()
                    .filter_map(|(i, &v)| if v % 3 == 1 { Some(i) } else { None })
                    .take(2)
                    .collect();
                for i in temp {
                    digits.remove(i);
                }
            }
        }
        _ => (),
    }
    let res: String = digits
        .into_iter()
        .map(|v| char::from(v as u8 + b'0'))
        .collect();
    if res.starts_with('0') {
        "0".to_string()
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(largest_multiple_of_three(vec![8, 1, 9]), "981");
        assert_eq!(largest_multiple_of_three(vec![8, 6, 7, 1, 0]), "8760");
        assert_eq!(largest_multiple_of_three(vec![1]), "");
    }

    #[test]
    fn test() {
        assert_eq!(largest_multiple_of_three(vec![1, 1, 1, 2]), "111");
        assert_eq!(largest_multiple_of_three(vec![5, 8]), "");
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
