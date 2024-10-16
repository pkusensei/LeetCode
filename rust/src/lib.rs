mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn asteroid_collision(nums: &[i32]) -> Vec<i32> {
    let mut stack: Vec<i32> = vec![];
    for &(mut num) in nums.iter() {
        while num < 0 && stack.last().is_some_and(|&v| v > 0) {
            let Some(v) = stack.pop() else {
                break;
            };
            match v.cmp(&num.abs()) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Equal => num = 0,
                std::cmp::Ordering::Greater => num = v,
            }
        }
        if num != 0 {
            stack.push(num);
        }
    }
    stack
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(asteroid_collision(&[5, 10, -5]), [5, 10]);
        debug_assert!(asteroid_collision(&[8, -8]).is_empty());
        debug_assert_eq!(asteroid_collision(&[10, 2, -5]), [10]);
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
