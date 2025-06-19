mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_covered_buildings(_n: i32, buildings: &[[i32; 2]]) -> i32 {
    use std::collections::HashMap;
    let mut col_minrs = HashMap::new();
    let mut col_maxrs = HashMap::new();
    let mut row_mincs = HashMap::new();
    let mut row_maxcs = HashMap::new();
    for b in buildings.iter() {
        let [x, y] = b[..] else { unreachable!() };
        let v = col_minrs.entry(x).or_insert(y);
        *v = (*v).min(y);
        let v = col_maxrs.entry(x).or_insert(y);
        *v = (*v).max(y);
        let v = row_mincs.entry(y).or_insert(x);
        *v = (*v).min(x);
        let v = row_maxcs.entry(y).or_insert(x);
        *v = (*v).max(x);
    }
    let mut res = 0;
    for b in buildings.iter() {
        let [x, y] = b[..] else { unreachable!() };
        res += i32::from(
            (1 + col_minrs[&x]..col_maxrs[&x]).contains(&y)
                && (1 + row_mincs[&y]..row_maxcs[&y]).contains(&x),
        )
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(
            count_covered_buildings(5, &[[1, 3], [3, 2], [3, 3], [3, 5], [5, 3]]),
            1
        );
    }

    #[test]
    fn test() {}
}
