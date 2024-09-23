mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn makesquare(matchsticks: &mut [i32]) -> bool {
    let n = matchsticks.len();
    if n < 4 {
        return false;
    }
    let sum: i32 = matchsticks.iter().sum();
    if sum == 0 || sum % 4 > 0 {
        return false;
    }
    matchsticks.sort_unstable_by_key(|&n| std::cmp::Reverse(n));
    let side = sum / 4;
    let mut sides = [0; 4];
    dfs(&mut sides, side, matchsticks)
}

fn dfs(sides: &mut [i32; 4], side: i32, nums: &[i32]) -> bool {
    match nums {
        [] => true,
        [head, tail @ ..] => {
            for i in 0..4 {
                if sides[i] + head <= side {
                    sides[i] += head;
                    if dfs(sides, side, tail) {
                        return true;
                    }
                    sides[i] -= head;
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

    #[test]
    fn basics() {
        debug_assert!(makesquare(&mut [1, 1, 2, 2, 2]));
        debug_assert!(!makesquare(&mut [3, 3, 3, 3, 4]));
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
