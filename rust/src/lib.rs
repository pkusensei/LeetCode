mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_num_of_substrings(s: &str) -> Vec<String> {
    let n = s.len();
    let mut intervals = [[n, n]; 26];
    for (i, b) in s.bytes().enumerate() {
        let idx = usize::from(b - b'a');
        intervals[idx][0] = intervals[idx][0].min(i);
        intervals[idx][1] = i;
    }
    let mut stack: Vec<[usize; 2]> = vec![];
    let mut res = vec![];
    for (i, b) in s.bytes().enumerate() {
        let idx = usize::from(b - b'a');
        // For valid substring, its leftmost idx must be the specific char.
        // Hence int[idx][0]==i => a possible valid substring
        if intervals[idx][0] != i {
            continue;
        }
        let left = intervals[idx][0];
        let tail = intervals[idx][1];
        // Find potential expansion
        let Some(right) = right_most(&intervals, s.as_bytes(), left, tail) else {
            continue;
        };
        // Pop potential "enclosing" interval
        while stack.last().is_some_and(|v| v[0] <= left && right <= v[1]) {
            stack.pop();
        }
        stack.push([left, right]);
    }
    while let Some(v) = stack.pop() {
        res.push(s[v[0]..=v[1]].to_string());
    }
    res
}

fn right_most(
    intervals: &[[usize; 2]; 26],
    s: &[u8],
    left: usize,
    mut right: usize,
) -> Option<usize> {
    let mut i = 1 + left;
    // For each i inside [1+left..right]
    // If a valid substring starts within, expand right-ward
    while i < right {
        let idx = usize::from(s[i] - b'a');
        if intervals[idx][0] < left {
            return None;
        }
        right = right.max(intervals[idx][1]);
        i += 1;
    }
    Some(right)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(max_num_of_substrings("adefaddaccc"), ["e", "f", "ccc"]);
        sort_eq(max_num_of_substrings("abbaccd"), ["d", "bb", "cc"]);
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
