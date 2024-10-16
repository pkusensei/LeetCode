mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let [sr, sc] = [sr, sc].map(|n| n as usize);
    let start = image[sr][sc];
    if start == color {
        return image;
    }
    let mut queue = std::collections::VecDeque::from([(sr, sc)]);
    while let Some((row, col)) = queue.pop_front() {
        if image[row][col] == color {
            continue;
        }
        image[row][col] = color;
        for (nr, nc) in neighbors((row, col)).filter(|&(nr, nc)| {
            image
                .get(nr)
                .is_some_and(|v| v.get(nc).is_some_and(|&num| num == start))
        }) {
            queue.push_back((nr, nc));
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            [[2, 2, 2], [2, 2, 0], [2, 0, 1]]
        );
        debug_assert_eq!(
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            [[0, 0, 0], [0, 0, 0]]
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
}
