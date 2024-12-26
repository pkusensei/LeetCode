mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_candies(
    status: &mut [i32],
    candies: &[i32],
    keys: &[&[i32]],
    contained_boxes: &[&[i32]],
    initial_boxes: &[i32],
) -> i32 {
    let n = status.len();
    let mut has_box = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    for &b in initial_boxes.iter() {
        if status[b as usize] == 1 {
            queue.push_back(b as usize);
        } else {
            has_box[b as usize] = true;
        }
    }
    let mut res = 0;
    while let Some(box_) = queue.pop_front() {
        res += candies[box_];
        for &k in keys[box_].iter() {
            status[k as usize] = 1;
        }
        for &b in contained_boxes[box_].iter() {
            has_box[b as usize] = true;
        }
        for b in has_box
            .iter()
            .zip(status.iter())
            .enumerate()
            .filter_map(|(b, (&has, &st))| if has && st == 1 { Some(b) } else { None })
            .collect::<Vec<_>>()
        {
            has_box[b] = false;
            queue.push_back(b);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // assert_eq!(
        //     max_candies(
        //         &mut [1, 0, 1, 0],
        //         &[7, 5, 4, 100],
        //         &[&[], &[], &[1], &[]],
        //         &[&[1, 2], &[3], &[], &[]],
        //         &[0]
        //     ),
        //     16
        // );
        assert_eq!(
            max_candies(
                &mut [1, 0, 0, 0, 0, 0],
                &[1, 1, 1, 1, 1, 1],
                &[&[1, 2, 3, 4, 5], &[], &[], &[], &[], &[]],
                &[&[1, 2, 3, 4, 5], &[], &[], &[], &[], &[]],
                &[0]
            ),
            6
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
