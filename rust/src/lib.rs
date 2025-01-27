mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_jumps(forbidden: &[i32], a: i32, b: i32, x: i32) -> i32 {
    let forbids: HashSet<_> = forbidden.iter().copied().collect();
    let max = a + b + forbidden.iter().copied().max().unwrap_or(x).max(x);
    // pos, backwards, step
    let mut queue = VecDeque::from([(0, false, 0)]);
    let mut seen = vec![vec![false; 1 + max as usize]; 2];
    while let Some((pos, backward, step)) = queue.pop_front() {
        if pos == x {
            return step;
        }
        let forepos = pos + a;
        if forepos <= max && !seen[0][forepos as usize] && !forbids.contains(&pos) {
            seen[0][forepos as usize] = true;
            queue.push_back((forepos, false, 1 + step));
        }
        let backpos = pos - b;
        if backpos >= 0 && !forbids.contains(&pos) && !backward && !seen[1][backpos as usize] {
            seen[1][backpos as usize] = true;
            queue.push_back((backpos, true, 1 + step));
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(minimum_jumps(&[1, 6, 2, 14, 5, 17, 4], 16, 9, 7), 2);
    }

    #[test]
    fn test() {}

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
