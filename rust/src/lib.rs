mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn optimal_division(nums: &[i32]) -> String {
    match nums.len() {
        0 => String::new(),
        1 => nums[0].to_string(),
        2 => format!("{}/{}", nums[0], nums[1]),
        _ => {
            let right = nums[1..]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("/");
            format!("{}/({})", nums[0], right)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(optimal_division(&[1000, 100, 10, 2]), "1000/(100/10/2)");
        debug_assert_eq!(optimal_division(&[2, 3, 4]), "2/(3/4)");
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
