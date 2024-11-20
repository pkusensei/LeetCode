mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn video_stitching(clips: &mut [[i32; 2]], time: i32) -> i32 {
    let time = time as usize;
    clips.sort_unstable();
    let mut dp = vec![101; 1 + time];
    dp[0] = 0;
    for clip in clips.iter() {
        let [a, b] = [clip[0], clip[1]].map(|v| v as usize);
        for i in 1 + a..=b.min(time) {
            dp[i] = dp[i].min(1 + dp[a])
        }
    }
    if dp[time] == 101 {
        -1
    } else {
        dp[time]
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            video_stitching(&mut [[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]], 10),
            3
        );
        debug_assert_eq!(video_stitching(&mut [[0, 1], [1, 2]], 5), -1);
        debug_assert_eq!(
            video_stitching(
                &mut [
                    [0, 1],
                    [6, 8],
                    [0, 2],
                    [5, 6],
                    [0, 4],
                    [0, 3],
                    [6, 7],
                    [1, 3],
                    [4, 7],
                    [1, 4],
                    [2, 5],
                    [2, 6],
                    [3, 4],
                    [4, 5],
                    [5, 7],
                    [6, 9]
                ],
                9
            ),
            3
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(video_stitching(&mut [[0, 4], [2, 8]], 5), 2)
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
