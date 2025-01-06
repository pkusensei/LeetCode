mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn min_jumps(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut map = HashMap::<_, Vec<_>>::new();
    for (i, w) in arr.windows(2).enumerate() {
        if w[0] == w[1] {
            continue;
        }
        map.entry(w[0]).or_default().push(i);
        map.entry(w[1]).or_default().push(i + 1);
    }
    map.entry(arr[n - 1]).or_default().push(n - 1);
    let mut queue = VecDeque::from([(0, 0)]);
    let mut seen = vec![false; n];
    seen[0] = true;
    while let Some((curr, dist)) = queue.pop_front() {
        if curr == n - 1 {
            return dist;
        }
        for &v in map[&arr[curr]].iter() {
            if !seen[v] {
                seen[v] = true;
                queue.push_back((v, 1 + dist));
            }
        }
        map.get_mut(&arr[curr]).map(|s| s.clear());
        if let Some(left) = curr.checked_sub(1).filter(|&v| !seen[v]) {
            seen[left] = true;
            queue.push_back((left, 1 + dist));
        }
        if let Some(right) = curr.checked_add(1).filter(|&v| v < n && !seen[v]) {
            seen[right] = true;
            queue.push_back((right, 1 + dist));
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_jumps(&[100, -23, -23, 404, 100, 23, 23, 23, 3, 404]), 3);
        assert_eq!(min_jumps(&[7]), 0);
        assert_eq!(min_jumps(&[7, 6, 9, 6, 9, 6, 9, 7]), 1);
    }

    #[test]
    fn test() {
        // assert_eq!(min_jumps(&[7, 7, 2, 1, 7, 7, 7, 3, 4, 1]), 3);
        assert_eq!(min_jumps(&[11, 7, 7, 7, 7, 7, 7]), 2);
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
