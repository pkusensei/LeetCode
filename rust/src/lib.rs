mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn validate_binary_tree_nodes(n: i32, left_child: &[i32], right_child: &[i32]) -> bool {
    let n = n as usize;
    let mut indegs = vec![0; n];
    for &node in left_child
        .iter()
        .chain(right_child.iter())
        .filter(|&&v| v >= 0)
    {
        indegs[node as usize] += 1;
    }
    let mut it = indegs
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v == 0 { Some(i) } else { None });
    let (Some(root), None) = (it.next(), it.next()) else {
        return false;
    };
    let mut seen = vec![false; n];
    seen[root] = true;
    let mut queue = std::collections::VecDeque::from([root]);
    while let Some(curr) = queue.pop_front() {
        for next in [left_child, right_child].map(|c| c[curr]) {
            if next < 0 {
                continue;
            }
            if seen[next as usize] {
                return false;
            }
            seen[next as usize] = true;
            queue.push_back(next as usize);
        }
    }
    seen.into_iter().all(|b| b)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(validate_binary_tree_nodes(
            4,
            &[1, -1, 3, -1],
            &[2, -1, -1, -1]
        ));
        assert!(!validate_binary_tree_nodes(
            4,
            &[1, -1, 3, -1],
            &[2, 3, -1, -1]
        ));
        assert!(!validate_binary_tree_nodes(2, &[1, 0], &[-1, -1]));
    }

    #[test]
    fn test() {
        assert!(validate_binary_tree_nodes(
            4,
            &[3, -1, 1, -1],
            &[-1, -1, 0, -1]
        ));
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
