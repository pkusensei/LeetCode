mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_moves_stones_ii(stones: &mut [i32]) -> Vec<i32> {
    stones.sort_unstable();
    let n = stones.len();
    // e.g [1,2,3,7]
    // move 1 to 4 => [2,3,4,7]
    // then 2 to 5 => [3,4,5,7] => [4,5,6,7]
    // i.e always move the first item stones[0]
    // hence the count is
    // 1) distance between 1 and 7 => A[n-1]-A[1]-1
    // 2) minus "free" items => n-3; the 3 are A[0], A[1], and A[n-1]
    let upper = ((stones[n - 1] - stones[1] - 1) - (n as i32 - 3))
        .max(stones[n - 2] - stones[0] - n as i32 + 2);
    let (mut left, mut lower) = (0, n);
    for (right, &num) in stones.iter().enumerate() {
        while num - stones[left] >= n as i32 {
            // The most between left and right is n
            left += 1;
        }
        if num - stones[left] == n as i32 - 2 && right - left + 1 == n - 1 {
            // window is consecutive && one stone is outside
            lower = lower.min(2);
        } else {
            // move all stones into this window
            lower = lower.min(n - (right - left + 1));
        }
    }
    vec![lower as i32, upper]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_moves_stones_ii(&mut [7, 4, 9]), [1, 2]);
        debug_assert_eq!(num_moves_stones_ii(&mut [6, 5, 4, 3, 10]), [2, 3]);
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
