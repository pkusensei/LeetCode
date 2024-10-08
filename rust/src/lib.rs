mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_swap(num: i32) -> i32 {
    let mut digits = num.to_string().into_bytes();
    // find last occurence of every digit
    let indices = digits.iter().enumerate().fold([0; 10], |mut acc, (i, &d)| {
        acc[usize::from(d - b'0')] = i;
        acc
    });
    let n = digits.len();
    for left in 0..n {
        // scan every digit from the left
        let digit = usize::from(digits[left] - b'0');
        // attempt to find a bigger digit
        for right in (1 + digit..=9).rev() {
            // if there is a bigger digit to the right
            if indices[right] > left {
                digits.swap(left, indices[right]);
                return String::from_utf8(digits).unwrap().parse().unwrap_or(num);
            }
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(maximum_swap(2736), 7236);
        debug_assert_eq!(maximum_swap(9973), 9973);
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
